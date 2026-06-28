use anyhow::Result;
use tokio::io::copy_bidirectional;
use tokio::net::TcpStream;
use tracing::info;

pub struct Relay;

impl Relay {
    pub async fn start(client: &mut TcpStream, remote: &mut TcpStream) -> Result<()> {
        let (upload, download) = copy_bidirectional(client, remote).await?;

        info!(
            "Relay finished: upload={} bytes, download={} bytes",
            upload, download
        );
        Ok(())
    }
}
