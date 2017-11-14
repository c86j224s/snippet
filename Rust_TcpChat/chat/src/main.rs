//use std::net::{TcpListener};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(v) => { println!("bound on 127.0.0.1:8888"); v },
        Err(e) => { panic!("binding failed. {:?}", e) }
    };

    let mut threads = vec![];
    //let mut clients_vec = vec![];
    //let clients : Arc<&mut Vec<Arc<TcpStream>>> = Arc::new(&mut clients_vec);
    let clients = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        let client = Arc::new(match stream {
            Ok(v) => {
                println!("new client accept! {:?}", v.peer_addr().unwrap());
                v 
            },
            Err(e) => { panic!("accepting failed. {:?}", e) }
        });

        {
            //Arc::make_mut(&mut clients).unwrap().push(Arc::clone(&client));
            clients.lock().unwrap().push(Arc::clone(&client));
        }

        let thread_client = Arc::clone(&client);
        let thread_clients = Arc::clone(&clients);
        let handle = thread::spawn(move || {
            let mut reader = BufReader::new(&*thread_client);

            loop {
                let mut line = String::new();
                let peer = match reader.read_line(&mut line) {
                    Ok(_) => {
                        let peer = Some(thread_client.peer_addr().unwrap());
                        println!("[{:?}] {}",
                            peer,
                            line.trim()
                        );
                        peer
                    },
                    Err(e) => {
                        println!("Error - [{:?}] read error. {:?}", 
                            thread_client.peer_addr().unwrap(),
                            e
                        );
                        break;
                    }
                };

                for it in thread_clients.lock().unwrap().iter() {
                    let cli = Arc::clone(&it);
                    let mut writer = BufWriter::new(&*cli);
                    write!(writer, "[{:?}] {}", peer, line);
                    match writer.flush() {
                        Err(e) => println!("Error - flush error. {:?}", e),
                        _ => {}
                    };
                }

                line.clear();
            }
            
            println!("[{:?}] disconnected.", thread_client.peer_addr().unwrap());
        });

        threads.push(handle);
    }

    for each in threads {
        match each.join() {
            Err(e) => { println!("Error - thread join error. {:?}", e) },
            _ => {}
        }
    }
}
