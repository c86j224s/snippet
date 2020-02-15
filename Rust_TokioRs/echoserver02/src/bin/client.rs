use bytes::BytesMut;

use protobuf::{parse_from_bytes, Message};

use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncReadExt};
use tokio::prelude::*;

use echoserver02::protos::AuthServer::{Packet, Packet_Id};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let mut request_packet = Packet::new();
    request_packet.set_id(Packet_Id::LOGIN);
    request_packet.mut_login().mut_account().set_login_name("gx3394".to_owned());
    request_packet.mut_login().mut_account().set_password("l-WhfSFrXZ0".to_owned());

    println!("request packet = {:?}", request_packet);
    
    let raw_request_packet = request_packet.write_to_bytes().unwrap();

    if let Err(e) = stream.write_u32(raw_request_packet.len() as u32).await {
        eprintln!("failed to write len to socket; err = {:?}", e);
        return Err(e)
    }

    if let Err(e) = stream.write_all(&raw_request_packet).await {
        eprintln!("failed to write packet to socket; err = {:?}", e);
        return Err(e)
    }

    let len = match stream.read_u32().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read len from socket; err = {:?}", e);
            return Err(e);
        }
    };

    let mut raw_response_packet = BytesMut::with_capacity(len as usize);
    let _n = match stream.read_buf(&mut raw_response_packet).await {
        Ok(n) if n == 0 => { println!("end of stream. (read 0 byte)"); return Ok(()); },
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed to read from socket; err= {:?}", e);
            return Err(e);
        }
    };

    let response_packet = match parse_from_bytes::<Packet>(&raw_response_packet) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("invalid login packet; err = {:?}", e);
            return Err(std::io::Error::new(io::ErrorKind::Other, e));
        }
    };

    println!("response = {:?}", response_packet);

    Ok(())
}