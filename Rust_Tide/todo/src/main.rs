use todo::handler;


#[async_std::main]
async fn main() -> std::io::Result<()> {
    handler::server_run().await
}
