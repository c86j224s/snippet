use std::{collections::BTreeMap, net::SocketAddr, ops::AddAssign, sync::{Arc, Mutex}};

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

use rand::Rng;

use crate::server_connection::{ServerConnection, SocketTaskMessage};

#[derive(Debug)]
pub enum ListenerMessage {
    Remove(u32),
    SendRandom(String), // random
}

pub struct Server {
    listener_tx: UnboundedSender<ListenerMessage>,
    shutdown_tx: watch::Sender<bool>,
    join_set: Arc<Mutex<JoinSet<()>>>,
}

impl Server {
    fn spawn_listener_task(
        addr: SocketAddr,
    ) -> Self {
        let join_set = Arc::new(Mutex::new(JoinSet::<()>::new()));

        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        let (tx, mut rx) = unbounded_channel::<ListenerMessage>();

        let listener_tx = tx.clone();
        join_set.lock().unwrap().spawn(async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            let mut next_socket_id = 1u32;
            let mut socket_handle_map =
                BTreeMap::<u32, ServerConnection>::new();

            let mut ticker = time::interval(time::Duration::from_secs(10));

            'the_loop: loop {
                select! {
                    r = shutdown_rx.changed() => {
                        match r {
                            Ok(_) => {
                                unimplemented!("shutdown listener and drop all tx and tasks");
                            }
                            Err(e) => {
                                println!("[container] shutdown receiver error. err[{:?}]", e);
                                break 'the_loop
                            }
                        }
                    }

                    r = listener.accept() => {
                        match r {
                            Ok((socket, _)) => {
                                let socket_id = next_socket_id;
                                next_socket_id += 1;

                                let conn = ServerConnection::spawn_socket_task(next_socket_id, socket, listener_tx.clone());

                                if let Some(prev) = socket_handle_map.insert(socket_id, conn) {
                                    println!("[container] new socket : duplicated socket. socket id[{}]", socket_id);
                                    drop(prev.tx);
                                    prev.t.await.unwrap();
                                }
                            },
                            Err(e) => {
                                println!("[listener] accept error, err[{:?}]", e);
                                break 'the_loop
                            }
                        }
                    }
                    r = rx.recv() => {
                        match r {
                            Some(ListenerMessage::Remove(socket_id)) => {
                                if let Some(prev) = socket_handle_map.remove(&socket_id) {
                                    println!("[listener] remove socket : socket_id[{}]", socket_id);
                                    drop(prev.tx);
                                    prev.t.await.unwrap();
                                }
                            }
                            Some(ListenerMessage::SendRandom(s)) => {
                                let selected = rand::thread_rng().gen_range(0..socket_handle_map.len());
                                if let Some((_, conn)) = socket_handle_map.iter().nth(selected) {
                                    if let Err(e) = conn.tx.send(SocketTaskMessage::Send(s)) {
                                        println!("[listener] failed to send request, err[{}]", e);
                                    }
                                }
                            }
                            None => {}
                        }
                    }

                    t = ticker.tick() => {
                        // TODO remove this
                        // this feature is for client
                    }
                }
            }

            rx.close();
            while let Some(message) = rx.recv().await {
                match message {
                    ListenerMessage::Remove(socket_id) => {
                        if let Some(prev) = socket_handle_map.remove(&socket_id) {
                            println!("[listener] remove socket : socket_id[{}]", socket_id);
                            drop(prev.tx);
                            prev.t.await.unwrap();
                        }
                    }
                    ListenerMessage::SendRandom(s) => {
                        let selected = rand::thread_rng().gen_range(0..socket_handle_map.len());
                        if let Some((_, conn)) = socket_handle_map.iter().nth(selected) {
                            if let Err(e) = conn.tx.send(SocketTaskMessage::Send(s)) {
                                println!("[listener] failed to send request, err[{}]", e);
                            }
                        }
                    }
                }
            }

            assert!(socket_handle_map.is_empty());
        });

        Self {
            listener_tx: tx,
            shutdown_tx,
            join_set,
        }
    }

    pub fn start(addr: SocketAddr) -> anyhow::Result<Self> {
        let this = Self::spawn_listener_task(addr);
        
        Ok(this)
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.shutdown_tx.send(true)?;

        while let Some(res) = self.join_set.lock().unwrap().join_next().await {
            res.unwrap();
        }

        Ok(())
    }

    pub fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.listener_tx
            .send(ListenerMessage::SendRandom(message))?;

        Ok(())
    }
}
