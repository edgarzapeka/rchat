use tokio::prelude::*;
use rchat::server::Server;
use rchat::error::Result;
use log::error;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await;
}
