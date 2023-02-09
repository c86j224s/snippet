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

    // 클라이언트 모두에게 메시지를 보낸다.
    pub fn send_broadcast(&self, message: String) -> anyhow::Result<()> {
        self.listener_tx.send(ListenerMessage::Broadcast(message))?;

        Ok(())
    }

    // 서버를 시작하고 돌리는 메인 로직이다.
    fn spawn_listener_task(addr: SocketAddr) -> Self {
        // joinset은 리스너 태스크 안으로 복사되어 넘어가야 하기 때문에 Arc + Mutex로 감싼다.
        let join_set = Arc::new(Mutex::new(JoinSet::<()>::new()));

        // 서버가 셧다운 되어야 함을 whatch 채널로 알려준다.
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // 서버는 rx로부터 받은 메시지를 핸들링한다.
        // tx는 서버 객체의 소유자와, accept된 소켓 태스크들이 복사하여 나눠 가진다.
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
                    // 셧다운 이벤트 대기
                    r = shutdown_rx.changed() => {
                        info!("[listener] shutdown request received. r[{:?}]", r);

                        break 'the_loop
                    }

                    // 새로운 접속 대기
                    r = listener.accept() => {
                        match r {
                            Ok((socket, _)) => {
                                let socket_id = next_socket_id;
                                next_socket_id += 1;

                                // 새로운 소켓 태스크를 스폰
                                let conn = ServerConnection::spawn_socket_task(socket_id, socket, listener_tx.clone());

                                // 그리고 소켓 맵에도 저장한다.
                                if let Some(mut prev) = socket_handle_map.insert(socket_id, conn) {
                                    info!("[container] new socket : duplicated socket. socket id[{}]", socket_id);
                                    prev.tx = None; // drop unbounded sender
                                    prev.t.await.unwrap();
                                }
                            },

                            // 새로운 접속을 받는데 싪패했다면 서버를 종료한다.
                            Err(e) => {
                                info!("[listener] accept error, err[{:?}]", e);
                                break 'the_loop
                            }
                        }
                    }

                    // 리스너 태스크에 오는 요청을 처리한다.
                    r = rx.recv() => {
                        if let Some(message) = r {
                            Self::listener_message_handler(&mut socket_handle_map, message).await;
                        }
                    }

                    // 주기적으로 연결된 소켓들에 PING을 보내준다.
                    _ = ticker.tick() => {
                        if let Err(e) = listener_tx.send(ListenerMessage::Broadcast("PING".to_owned())) {
                            error!("[listener] on tick, failed to broadcast ping, err[{}]", e);
                        }
                    }
                }
            }

            // 리스너 태스크 종료 시작.

            // 소켓 맵의 모든 소켓들의 tx를 drop해서 종료가 시작되었음을 알린다.
            for (_, s) in socket_handle_map.iter_mut() {
                s.tx = None; // drop unbounded sender
            }

            // 이렇게 sleep 걸어서 기다리는 것도 올바르지 않지만, 일단 2초 슬립해서 소켓 태스크들이
            // 종료를 처리할 시간을 준다.
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            // 리스너 태스크의 rx를 닫아서 새로운 요청을 받지 않겠다고 알린 후 받은 나머지
            // 메시지들을 모두 처리한다.
            rx.close();
            while let Some(message) = rx.recv().await {
                Self::listener_message_handler(&mut socket_handle_map, message).await;
            }

            debug_assert!(socket_handle_map.is_empty());
        });

        Self {
            listener_tx: tx,
            shutdown_tx,
            join_set,
        }
    }

    // 받은 메시지를 처리한다.
    async fn listener_message_handler(
        socket_handle_map: &mut BTreeMap<u32, ServerConnection>,
        message: ListenerMessage,
    ) {
        match message {
            // 소켓 맵에서 소켓 정리를 요청받는다. 주로 소켓 연결이 끊어졌을 때 온다.
            ListenerMessage::Remove(socket_id) => {
                if let Some(mut prev) = socket_handle_map.remove(&socket_id) {
                    info!("[listener] remove socket : socket_id[{}]", socket_id);
                    prev.tx = None;
                    prev.t.await.unwrap();
                }
            }

            // 랜덤하게 어느 한 소켓에 메시지를 보내달라고 할 때 SendRandom을 사용.
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

            // 모든 소켓에 메시지를 보내달라고 할 때 Broadcast를 사용.
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
