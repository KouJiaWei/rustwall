use anyhow::Result;
use tokio::net::TcpStream;

use super::OutboundType;
use super::direct::DirectOutbound;
use crate::app::context::AppContext;

pub struct OutboundManager;

impl OutboundManager {
    pub async fn connect(
        ctx: &AppContext,
        outbound: OutboundType,
        host: &str,
        port: u16,
    ) -> Result<TcpStream> {
        match outbound {
            OutboundType::Direct => DirectOutbound::connect(ctx, host, port).await,
        }
    }
}
