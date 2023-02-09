use std::{net::SocketAddr, str::FromStr, time::Duration};

use gostyle::server::Server;
use tokio::time;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Hello, world on 6605 port!");

    let srv = Server::start(SocketAddr::from_str("0.0.0.0:6605")?)?;

    time::sleep(Duration::from_secs(30)).await;

    srv.stop().await?;

    Ok(())
}
