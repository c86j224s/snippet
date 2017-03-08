extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Framed};

pub struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer
            let line = buf.drain_to(i);

            // also, remove the '\n'
            buf.drain_to(i);

            // turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8"))
            }
        }
        else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.as_byte());
        buf.push(b'\n');
        Ok(())
    }
}


pub struct LineProto;

impl<T: Io + 'static> ServerProto<T> for LineProto {
    // For this protocol style, 'Request' matches the codec 'In' type.
    type Request = String;

    // For this protocol style, 'Response' matches the codec 'Out' type.
    type Response = String;

    // A bit of boilerplate to hook in  the codec.
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}


// todo



fn main() {
    println!("Hello, world!");
}
