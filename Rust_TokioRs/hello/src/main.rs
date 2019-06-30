extern crate hello;
extern crate tokio;

use std::env;
use tokio::prelude::*;

fn run_server(server_ip : &str, server_port : u32) -> std::io::Result<()>
{
    let addr = format!("{}:{}", server_ip, server_port);
    let addr = addr.parse().unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().map_err(|err| {
        println!("accept error = {:?}", err);
    }).for_each(|socket| {
        println!("{:?}", socket);

        let (rd, wr) = socket.split();
        let copied = tokio::io::copy(rd, wr).map(|amount| {
            println!("wrote {:?} bytes", amount)
        }).map_err(|err| {
            println!("io error {:?}", err)
        });

        tokio::spawn(copied)
    });

    println!("server listening on {}:{}", server_ip, server_port);

    tokio::run(server);

    Ok(())
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let filename = if args.len() < 2 {
        "sample.json"
    } else {
        args[1].as_str()
    };

    let loaded = hello::Sample::new_from_file(filename);

    println!("server : {} : {}", loaded.server_ip(), loaded.server_port());
    for account in loaded.accounts().iter() {
        println!("account - {} / {}", account.login_name(), account.password());
    }

    match run_server(loaded.server_ip(), loaded.server_port()) {
        Err(e) => panic!("{}", e),
        Ok(_) => ()
    }

}
