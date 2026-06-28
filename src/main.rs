mod app;
mod config;
mod dispatcher;
mod dns;
mod handler;
mod inbound {
    pub mod socks5;
}
mod outbound;
mod rule;
mod session;
mod transport;

use anyhow::Result;
use app::builder::AppBuilder;
use handler::handle_client;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::load_config()?;

    tracing_subscriber::fmt()
        .with_env_filter(&cfg.log_level)
        .init();

    info!("config = {:?}", cfg);

    let ctx = AppBuilder::build(&cfg)?;
    let listener = TcpListener::bind(&cfg.listen).await?;
    info!("Server listening on {}", cfg.listen);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        let ctx = ctx.clone();
        tokio::spawn(async move {
            handle_client(socket, addr, ctx).await;
        });
    }
}
