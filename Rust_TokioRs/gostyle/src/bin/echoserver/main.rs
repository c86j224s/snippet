use std::{net::SocketAddr, str::FromStr, time::Duration};

use gostyle::server::Server;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let srv = Server::start(SocketAddr::from_str("127.0.0.1:6605")?)?;

    time::sleep(Duration::from_secs(10)).await;

    srv.stop().await?;

    Ok(())
}
