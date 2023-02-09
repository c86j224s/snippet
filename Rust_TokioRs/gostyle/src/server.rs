use std::{
    collections::BTreeMap,
    net::SocketAddr,
    ops::AddAssign,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        watch,
    },
    task::{self, JoinHandle, JoinSet},
    time,
};
use tracing::{debug, error, info};

use rand::Rng;

use crate::server_connection::{ServerConnection, SocketTaskMessage};

#[derive(Debug)]
pub enum ListenerMessage {
    Remove(u32),
    SendRandom(String),
    Broadcast(String),
}

pub struct Server {
    listener_tx: UnboundedSender<ListenerMessage>,
    shutdown_tx: watch::Sender<bool>,
    join_set: Arc<Mutex<JoinSet<()>>>,
}

impl Server {
    // 서버를 실행한다.
    pub fn start(addr: SocketAddr) -> anyhow::Result<Self> {
        info!("[listener] start on {}", &addr);

        let this = Self::spawn_listener_task(addr);

        Ok(this)
    }

    // 서버를 중단하고, 중단이 완료될 때까지 기다린다.
    pub async fn stop(self) -> anyhow::Result<()> {
        info!("[listener] shutting down");

        self.shutdown_tx.send(true)?;

        while let Some(res) = self.join_set.lock().unwrap().join_next().await {
            res.unwrap();
        }

        info!("[listener] shutdown complete");

        Ok(())
    }

    // 클라이언트 중 어느 하나에게 메시지를 보낸다.
    pub fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.listener_tx
            .send(ListenerMessage::SendRandom(message))?;

        Ok(())
    }

    pub fn send_broadcast(&self, message: String) -> anyhow::Result<()> {
        self.listener_tx.send(ListenerMessage::Broadcast(message))?;

        Ok(())
    }

    // 서버를 시작하고 돌리는 메인 로직이다.
    fn spawn_listener_task(addr: SocketAddr) -> Self {
        let join_set = Arc::new(Mutex::new(JoinSet::<()>::new()));

        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        let (tx, mut rx) = unbounded_channel::<ListenerMessage>();

        let listener_tx = tx.clone();
        join_set.lock().unwrap().spawn(async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            let mut next_socket_id = 1u32;
            let mut socket_handle_map =
                BTreeMap::<u32, ServerConnection>::new();

            let mut ticker = time::interval(time::Duration::from_secs(30));

            'the_loop: loop {
                select! {
                    r = shutdown_rx.changed() => {
                        info!("[listener] shutdown request received. r[{:?}]", r);

                        break 'the_loop
                    }

                    r = listener.accept() => {
                        match r {
                            Ok((socket, _)) => {
                                let socket_id = next_socket_id;
                                next_socket_id += 1;

                                let conn = ServerConnection::spawn_socket_task(socket_id, socket, listener_tx.clone());

                                if let Some(prev) = socket_handle_map.insert(socket_id, conn) {
                                    info!("[container] new socket : duplicated socket. socket id[{}]", socket_id);
                                    drop(prev.tx);
                                    prev.t.await.unwrap();
                                }
                            },
                            Err(e) => {
                                info!("[listener] accept error, err[{:?}]", e);
                                break 'the_loop
                            }
                        }
                    }
                    r = rx.recv() => {
                        if let Some(message) = r {
                            Self::listener_message_handler(&mut socket_handle_map, message).await;
                        }
                    }

                    _ = ticker.tick() => {
                        if let Err(e) = listener_tx.send(ListenerMessage::Broadcast("PING".to_owned())) {
                            error!("[listener] on tick, failed to broadcast ping, err[{}]", e);
                        }
                    }
                }
            }

            for (_, s) in socket_handle_map.iter_mut() {
                s.tx = None; // drop unbounded sender
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            rx.close();
            while let Some(message) = rx.recv().await {
                Self::listener_message_handler(&mut socket_handle_map, message).await;
            }

            if !socket_handle_map.is_empty() {
                for (idx, s) in socket_handle_map.iter() {
                    debug!("socket not removed : idx[{}], s[{:?}]", idx, &s);
                }
                panic!("socket_handle_map is not empty, len[{}]", socket_handle_map.len());
            }
        });

        Self {
            listener_tx: tx,
            shutdown_tx,
            join_set,
        }
    }

    async fn listener_message_handler(
        socket_handle_map: &mut BTreeMap<u32, ServerConnection>,
        message: ListenerMessage,
    ) {
        match message {
            ListenerMessage::Remove(socket_id) => {
                if let Some(mut prev) = socket_handle_map.remove(&socket_id) {
                    info!("[listener] remove socket : socket_id[{}]", socket_id);
                    prev.tx = None;
                    prev.t.await.unwrap();
                }
            }
            ListenerMessage::SendRandom(s) => {
                debug!("[listener] received SendRandom, s[{}]", &s);
                let selected = rand::thread_rng().gen_range(0..socket_handle_map.len());
                if let Some((_, conn)) = socket_handle_map.iter().nth(selected) {
                    if let Some(conn_tx) = &conn.tx {
                        if let Err(e) = conn_tx.send(SocketTaskMessage::Send(s)) {
                            error!("[listener] failed to send request, err[{}]", e);
                        }
                    }
                }
            }
            ListenerMessage::Broadcast(s) => {
                debug!("[listener] received Broadcast,  s[{}]", &s);
                for (_, conn) in socket_handle_map.iter() {
                    if let Some(conn_tx) = &conn.tx {
                        if let Err(e) = conn_tx.send(SocketTaskMessage::Send(s.clone())) {
                            error!("[listener] failed to send request, err[{}]", e);
                        }
                    }
                }
            }
        }
    }
}
