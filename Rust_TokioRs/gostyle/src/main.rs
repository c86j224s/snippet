use std::{collections::BTreeMap, net::SocketAddr};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        watch,
    },
    task::{self, JoinHandle},
    time,
};

use rand::Rng;

#[derive(Debug)]
enum ListenerMessage {
    Remove(u32),
    SendRandom(String), // random
}

enum SocketTaskMessage {
    Send(String),
}

struct Server {
    listener_handle: JoinHandle<()>,
    listener_tx: UnboundedSender<ListenerMessage>,
    shutdown_tx: watch::Sender<bool>,
}

impl Server {
    fn spawn_socket_task(
        socket_id: u32,
        mut socket: TcpStream,
        listener_tx: UnboundedSender<ListenerMessage>,
    ) -> (UnboundedSender<SocketTaskMessage>, JoinHandle<()>) {
        let (tx, mut rx) = unbounded_channel::<SocketTaskMessage>();

        let t = task::spawn(async move {
            let (rd, wr) = socket.split();

            let mut reader = BufReader::new(rd);
            let mut line = String::new();

            'the_loop: loop {
                select! {
                    r = reader.read_line(&mut line) => {
                        match r {
                            Ok(n) if n == 0 => {
                                // eof
                                // TODO
                            }
                            Ok(n) => {
                                // TODO
                            }
                            Err(e) => {
                                println!("[socket({})] error, err[{}]", socket_id, e);
                                if let Err(e) = listener_tx.send(ListenerMessage::Remove(socket_id)) {
                                    println!("[socket({})] failed to send remove socket request, err[{}]", socket_id, e);
                                }
                                break 'the_loop
                            }
                        }
                    }

                    r = rx.recv() => {
                        match r {
                            Some(SocketTaskMessage::Send(s)) => {

                            }
                            None => {
                                println!("[socket({})] channel closed", socket_id);
                                if let Err(e) = listener_tx.send(ListenerMessage::Remove(socket_id)) {
                                    println!("[socket[{}] failed to send remove socket request, err[{}]", socket_id, e);
                                }
                                break 'the_loop
                            }
                        }
                    }
                }
            }
        });

        (tx, t)
    }

    fn spawn_listener_task(
        addr: SocketAddr,
        mut shutdown_rx: watch::Receiver<bool>,
    ) -> (UnboundedSender<ListenerMessage>, JoinHandle<()>) {
        let (tx, mut rx) = unbounded_channel::<ListenerMessage>();

        let listener_tx = tx.clone();
        let t = task::spawn(async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            let mut next_socket_id = 1u32;
            let mut socket_handle_map =
                BTreeMap::<u32, (UnboundedSender<SocketTaskMessage>, JoinHandle<()>)>::new();

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

                                let (socket_tx, t) = Self::spawn_socket_task(next_socket_id, socket, listener_tx.clone());

                                if let Some((prev_sender, prev_t)) = socket_handle_map.insert(socket_id, (socket_tx, t)) {
                                    println!("[container] new socket : duplicated socket. socket id[{}]", socket_id);
                                    drop(prev_sender);
                                    prev_t.await.unwrap();
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
                                if let Some((prev_sender, prev_t)) = socket_handle_map.remove(&socket_id) {
                                    println!("[listener] remove socket : socket_id[{}]", socket_id);
                                    drop(prev_sender);
                                    prev_t.await.unwrap();
                                }
                            }
                            Some(ListenerMessage::SendRandom(s)) => {
                                let selected = rand::thread_rng().gen_range(0..socket_handle_map.len());
                                if let Some((_, (socket_tx, _))) = socket_handle_map.iter().nth(selected) {
                                    if let Err(e) = socket_tx.send(SocketTaskMessage::Send(s)) {
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
                        if let Some((prev_sender, prev_t)) = socket_handle_map.remove(&socket_id) {
                            println!("[listener] remove socket : socket_id[{}]", socket_id);
                            drop(prev_sender);
                            prev_t.await.unwrap();
                        }
                    }
                    ListenerMessage::SendRandom(s) => {
                        let selected = rand::thread_rng().gen_range(0..socket_handle_map.len());
                        if let Some((_, (socket_tx, _))) = socket_handle_map.iter().nth(selected) {
                            if let Err(e) = socket_tx.send(SocketTaskMessage::Send(s)) {
                                println!("[listener] failed to send request, err[{}]", e);
                            }
                        }
                    }
                }
            }

            assert!(socket_handle_map.is_empty());
        });

        (tx, t)
    }

    pub fn start(addr: SocketAddr) -> anyhow::Result<Self> {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let (tx, t) = Self::spawn_listener_task(addr, shutdown_rx);

        Ok(Self {
            listener_handle: t,
            listener_tx: tx,
            shutdown_tx,
        })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.shutdown_tx.send(true)?;
        self.listener_handle.await?;

        Ok(())
    }

    pub fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.listener_tx
            .send(ListenerMessage::SendRandom(message))?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    Ok(())
}
