use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Write};

fn main() {
    let stream = match TcpStream::connect("kr.ncsoft.com:80") {
        Ok(stream) => stream,
        Err(err) => panic!("connect fail : {:?}", err)
    };

    println!("connected.");

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    write!(writer, "GET / HTTP/1.1\r\n");
    write!(writer, "Host: kr.ncsoft.com\r\n");
    write!(writer, "Content-Length: 0\r\n\r\n");

    match writer.flush() {
        Ok(_) => {},
        Err(err) => panic!("writer flush fail : {:?}", err)
    }

    println!("request sent.");

    let mut response = String::new();
    loop {
        match reader.read_line(&mut response) {
            Ok(len) => {
                if len == 0 {
                    println!("eof.");
                    break;
                }
            },
            Err(err) => {
                panic!("read line fail : {:?}", err)
            }
        }

        if response == "\r\n" {
            break;
        }

        println!("{}", response.trim());
        response.clear();
    }
}
