// ref to ... https://rinthel.github.io/rust-lang-book-ko/ch20-00-final-project-a-web-server.html

use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs;
use std::thread;
use std::vec;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut handlers : vec::Vec<thread::JoinHandle<_>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handlers.push(thread::spawn(move || {
            handle_connection(stream);
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) {
    // max read  ~ 512 bytes
    let mut buffer =[0; 512];
    stream.read(&mut buffer).unwrap();

    // parse request line
    // only supports GET method
    let request_line = buffer.split(|ch| *ch == b'\r').next().unwrap();
    let mut iter = request_line.split(|ch| *ch == b' ');

    let _method = iter.next().unwrap();
    let uri = iter.next().unwrap();
    let _version = iter.next().unwrap();

    // matching response
    let (status_line, content_type, file_name) = if uri == b"/" {
        ("HTTP/1.1 200 OK \r\n", "text/html", String::from("index.html"))
    } else if uri.starts_with(b"/static/") {
        // caution!! insecure
        ("HTTP/1.1 200 OK \r\n", "image/jpeg", String::from_utf8_lossy(&uri[8..]).to_string())
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "text/html", String::from("404.html"))
    };

    let file_content = fs::read(file_name).unwrap();

    // send outputs
    stream.write(status_line.as_bytes()).unwrap();
    stream.write(format!("Content-Length: {}\r\n", file_content.len()).as_bytes()).unwrap();
    stream.write(format!("Content-Type: {}\r\n", content_type).as_bytes()).unwrap();
    stream.write(b"\r\n").unwrap();
    stream.write(&file_content).unwrap();
    stream.flush().unwrap();
}
