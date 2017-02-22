extern crate reqwest;

use std::io::*;
use std::convert::*;

fn fetch_page(hostname : &'static str, path : &'static str) -> String {
    let addr = format!("https://{}{}", hostname, path);
    //println!("addr : {}", addr);

    let cli = reqwest::Client::new().unwrap();
    let mut resp = cli.get(addr).header(
        reqwest::header::Host {
            hostname: "torrentkim5.net".to_owned(),
            port: None
        }).send().unwrap();

    println!("{}", resp.status());
    assert!(resp.status().is_success());
    
    let mut body = String::new();
    let bodylen = resp.read_to_string(&mut body).unwrap();
    println!("body = {:?} bodylen = {:?}", body, bodylen);

    body
}

   

fn main() {
    println!("Hello, world!");

    fetch_page("torrentkim5.net", "/torrent_variety/torrent1.htm");
}
