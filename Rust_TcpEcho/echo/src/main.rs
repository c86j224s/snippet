use std::net::{TcpListener};
use std::io::{BufReader, BufWriter, BufRead, Write};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(v) => { println!("127.0.0.1:8888 binded."); v },
        Err(e) => { panic!("bind failed : {:?}", e) }
    };

    for stream in listener.incoming() {
        let client = match stream {
            Ok(v) => { println!("new client accepted! {:?}", v.peer_addr().unwrap()); v },
                Err(e) => { panic!("accept failed : {:?}", e); }
        };

        let mut reader = BufReader::new(&client);
        let mut writer = BufWriter::new(&client);

        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Ok(len) => { 
                    println!("read : {} chars, {}", len, line);
                    len
                },
                Err(e) => {
                    println!("read error! {:?}", e);
                    break;
                }
            };

            write!(writer, "{}", line);
            writer.flush();

            line.clear();
        }
    }


}
