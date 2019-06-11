/*
 *  Rust std reference만 보고 쓰레드풀 채팅 서버 만들기 도전 !!
 *
 *  2019.06.11. chat에서 fork(?)함.
 *
 */

//use std::net::{TcpListener};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Write, Read, ErrorKind};
use std::thread;
use std::sync::{Arc, Mutex};


/*
fn main0() {
    // 리스너 바인딩.
    let listener : TcpListener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(v) => { println!("bound on 127.0.0.1:8888"); v },
        Err(e) => { panic!("binding failed. {:?}", e) }
    };

    // 쓰레드들의 핸들을 종료 시에 Join 하기 위해 관리해준다.
    let mut threads = vec![];

    // 클라이언트들(TcpStream)을 관리해준다. 나중에 이걸로 채팅을 뿌려줘야 한다.
    // @TODO: 아래는 최종 코드에서는 지워야 할 주석임. 만들면서 중간의 여러 삽질들...
    //let mut clients_vec = vec![];
    //let clients : Arc<&mut Vec<Arc<TcpStream>>> = Arc::new(&mut clients_vec);
    let clients = Arc::new(Mutex::new(vec![]));

    // 들어오는 커넥션들을 받아준다. 계속 들어올 수 있으므로 루프 돈다.
    for stream in listener.incoming() {
        // 방금 들어온 커넥션의 TcpStream을 레퍼런스 카운터로 감싸준다.
        let client : Arc<TcpStream> = Arc::new(match stream {
            Ok(v) => {
                println!("new client accept! {:?}", v.peer_addr().unwrap());
                v 
            },
            Err(e) => { panic!("accepting failed. {:?}", e) }
        });

        // 전체 클라이언트 커넥션 목록에 방금 들어온 커넥션을 추가해준다.
        //Arc::make_mut(&mut clients).unwrap().push(Arc::clone(&client));
        clients.lock().unwrap().push(Arc::clone(&client));

        // 방금 들어온 커넥션의 IO 쓰레드를 만들어준다. 커넥션들은 레퍼런스 카운터 올려서 넘겨준다.
        let thread_client = Arc::clone(&client);
        let thread_clients = Arc::clone(&clients);
        // handle : JoinHandle<T>
        let handle = thread::spawn(move || {
            // BufReader로 받아서 수신 루프 돌려줌.
            let mut reader = BufReader::new(&*thread_client);
            let peer = thread_client.peer_addr().unwrap();

            loop {
                // 읽어서.. 서버에 뿌려주고..
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(_) => {
                        println!("[{:?} said] {}", peer, line.trim());
                    },
                    Err(e) => {
                        println!("[{:?} error] read error. {:?}", peer, e);

                        // read_line에서 실패했으면, 그냥 끊어버린다.
                        let idx = thread_clients
                            .lock()
                            .unwrap()
                            .iter()
                            .position(|r| (*r).peer_addr().unwrap() == thread_client.peer_addr().unwrap())
                            .unwrap();
                        thread_clients.lock().unwrap().remove(idx);
                        println!("[{:?} error] server disconnected", peer);
                        break;
                    }
                };

                // 다른 피어들에게 뿌려준다.
                for it in thread_clients.lock().unwrap().iter() {
                    let another_peer = Arc::clone(&it);
                    let mut writer = BufWriter::new(&*another_peer);
                    write!(writer, "[{:?} said] {}", peer, line);
                    match writer.flush() {
                        Err(e) => println!("Error - flush error. {:?}", e),
                        _ => {}
                    };
                }

                line.clear();
            }
            
            println!("[{:?} event] disconnected.", thread_client.peer_addr().unwrap());
        });

        // IO 쓰레드는 전체 쓰레드 핸들 목록에 추가해줌..
        threads.push(handle);
    }

    // 종료할 땐 다 잘 닫아주자..
    for each in threads {
        match each.join() {
            Err(e) => { println!("Error - thread join error. {:?}", e) },
            _ => {}
        }
    }
}
*/

fn main() {
    // 리스너 바인딩.
    let listener : TcpListener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(v) => { println!("bound on 127.0.0.1:8888"); v },
        Err(e) => { panic!("binding failed. {:?}", e) }
    };
   
    for stream in listener.incoming() {
        let mut client : TcpStream = match stream {
            Ok(v) => {
                println!("new client accept! {:?}", v.peer_addr().unwrap());
                v 
            },
            Err(e) => { panic!("accepting failed. {:?}", e) }
        };

        // 잘 안돌아감~ 공부 좀더 해야 할 듯..

        match client.set_nonblocking(true) {
            Ok(_) => {}
            Err(e) => { panic!("set nonblocking error. {:?}", e) }
        }

        loop {
            client.take()
            let mut s = String::new();
            match client.read_to_string(&mut s) {
                Ok(read_size) => { println!("{}", s); },
                Ok(read_size) if read_size <= 0 => { println!("disconnected"); break; },
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
                Err(ref e)  => { panic!("read to string error. {:?}", e) }
            }
        }
    }
}
