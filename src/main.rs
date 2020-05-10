use tokio::prelude::*;
use rchat::server::Server;
use rchat::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await
}
