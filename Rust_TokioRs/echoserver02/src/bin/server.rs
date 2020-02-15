// TODOs
// - add tests
// - add a feature stopping the server
// - add common packet id-handler mapper using macro.

use bytes::BytesMut;

use protobuf::{parse_from_bytes, Message};

use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncReadExt};
use tokio::prelude::*;

use echoserver02::protos::AuthServer::{Packet, Packet_Id};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (rd, mut wr) = socket.split();
            let mut reader = BufReader::new(rd);

            loop {
                let len = match reader.read_u32().await {
                    Ok(v) => v,
                    Err(e) => { 
                        eprintln!("failed to read len from socket; err = {:?}", e);
                        return;
                    }
                };

                let mut raw_packet = BytesMut::with_capacity(len as usize);
                let _n = match reader.read_buf(&mut raw_packet).await {
                    Ok(n) if n == 0 => { println!("end of stream. (read 0 byte)"); return; },
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err= {:?}", e);
                        return;
                    }
                };

                let mut packet = match parse_from_bytes::<Packet>(&raw_packet) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("invalid login packet; err = {:?}", e);
                        continue;
                    }
                };
                
                println!("request packet : {:?}", packet);

                // TODO: build packet id-handler mapper here.

                if packet.id != Packet_Id::LOGIN || !packet.has_login() {
                    eprintln!("request packet is need to be login packet");
                    continue;
                }

                packet.mut_login().mut_account().set_user_id("123456".to_owned());
                
                let raw_response_packet = packet.write_to_bytes().unwrap();

                if let Err(e) = wr.write_u32(raw_response_packet.len() as u32).await {
                    eprintln!("failed to write len to socket; err = {:?}", e);
                    return;
                }

                if let Err(e) = wr.write_all(&raw_response_packet).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}


