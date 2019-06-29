extern crate hello;
extern crate tokio;

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
    let loaded = hello::Sample::new_from_file("sample.json");

    println!("server : {} : {}", loaded.server_ip(), loaded.server_port());
    for account in loaded.accounts().iter() {
        println!("account - {} / {}", account.login_name(), account.password());
    }

    match run_server(loaded.server_ip(), loaded.server_port()) {
        Err(e) => panic!("{}", e),
        Ok(_) => ()
    }

}
