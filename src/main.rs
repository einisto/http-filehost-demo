mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::create_server().await?;

    Ok(())
}
