use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
    select,
    sync::mpsc::{unbounded_channel, UnboundedSender},
    task::{self, JoinHandle},
};
use tracing::{error, info};

use crate::server::ListenerMessage;

pub enum SocketTaskMessage {
    Send(String),
}

#[derive(Debug)]
pub struct ServerConnection {
    pub tx: Option<UnboundedSender<SocketTaskMessage>>,
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
            let (rd, mut wr) = socket.split();

            let mut reader = BufReader::new(rd);

            'the_loop: loop {
                let mut line = String::new();

                select! {
                    r = reader.read_line(&mut line) => {
                        match r {
                            Ok(n) if n == 0 => {
                                info!("[socket({})] eof.", socket_id);
                                break 'the_loop
                            }
                            Ok(_) => {
                                info!("[socket({})] read line : {}", socket_id, &line);

                                if let Err(e) = wr.write(line.as_bytes()).await {
                                    error!("[socket({})] failed to send to socket, message[{}], err[{}]", socket_id, line, e);
                                    break 'the_loop
                                }
                            }
                            Err(e) => {
                                error!("[socket({})] failed to read, err[{}]", socket_id, e);
                                break 'the_loop
                            }
                        }
                    }

                    r = rx.recv() => {
                        match r {
                            Some(SocketTaskMessage::Send(s)) => {
                                if let Err(e) = wr.write(s.as_bytes()).await {
                                    error!("[socket({})] failed to responed to socket, message[{}], err[{}]", socket_id, s, e);
                                    break 'the_loop
                                }
                            }
                            None => {
                                info!("[socket({})] channel closed", socket_id);
                                break 'the_loop
                            }
                        }
                    }
                }
            }

            if let Err(e) = listener_tx.send(ListenerMessage::Remove(socket_id)) {
                error!(
                    "[socket[{}] failed to send remove socket request, err[{}]",
                    socket_id, e
                );
            }
        });

        Self { tx: Some(tx), t }
    }
}
