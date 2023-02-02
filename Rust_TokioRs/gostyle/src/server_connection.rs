use tokio::{sync::mpsc::{UnboundedSender, unbounded_channel}, task::{JoinHandle, self}, net::TcpStream, io::{BufReader, AsyncBufReadExt}, select};

use crate::server::ListenerMessage;


pub enum SocketTaskMessage {
    Send(String),
}

pub struct ServerConnection {
    pub tx: UnboundedSender<SocketTaskMessage>,
    pub t: JoinHandle<()>,
}

impl ServerConnection {
    pub fn spawn_socket_task(
        socket_id: u32,
        mut socket: TcpStream,
        listener_tx: UnboundedSender<ListenerMessage>,
    ) -> Self {
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
                                println!("read line : {}", &line);
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

        Self {
            tx,
            t,
        }
    }
}