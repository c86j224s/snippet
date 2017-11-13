use std::net::{TcpListener};
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::thread;

    let listener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(v) => { println!("127.0.0.1:8888 binded."); v },
        Err(e) => { panic!("bind failed : {:?}", e) }
    };

    let mut threads = vec![];

    for stream in listener.incoming() {
        let client = match stream {
            Ok(v) => { println!("new client accepted! {:?}", v.peer_addr().unwrap()); v },
                Err(e) => { panic!("accept failed : {:?}", e); }
        };

        let handle = thread::spawn(move || {
            let mut reader = BufReader::new(&client);
            let mut writer = BufWriter::new(&client);

            let mut line = String::new();
            loop {
                match reader.read_line(&mut line) {
                    Ok(_) => { 
                        println!("[{:?}] {}", 
                                client.peer_addr().unwrap(), 
                                line.trim());
                    },
                    Err(e) => {
                        println!("[LOG][ERROR] read error! {:?}", e);
                        break;
                    }
                };

                write!(writer, "{}", line);
                match writer.flush() {
                    Err(e) => { println!("[LOG][ERROR] flush error. {:?}", e); },
                    _ => {}
                }

                line.clear();
            }

            println!("client disconnected. {:?}", client.peer_addr().unwrap());
        });

        threads.push(handle);
    }

    for each in threads {
        match each.join() {
            Err(e) => { println!("[LOG][ERROR] thread join error. {:?}", e); },
            _ => {}
        }
    }
}
