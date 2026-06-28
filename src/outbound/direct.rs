use anyhow::{Context, Result};
use tokio::net::TcpStream;
use tracing::{error, info};

use crate::app::context::AppContext;

/// Direct 出站
///
/// 当前实现：
///
/// Client
///   ↓
/// rustwall
///   ↓
/// DirectOutbound
///   ↓
/// Internet
///
/// 后续会扩展：
///
/// Socks5Outbound
/// ShadowsocksOutbound
/// VlessOutbound
///

pub struct DirectOutbound;

impl DirectOutbound {
    pub async fn connect(ctx: &AppContext, host: &str, port: u16) -> Result<TcpStream> {
        let target = format!("{}:{}", host, port);
        info!("DirectOutbound connecting to {}", target);

        let addrs = ctx.resolver.resolve(host, port).await?;

        info!("DNS resolve {} => {:?}", host, addrs);

        let addr = addrs
            .first()
            .ok_or_else(|| anyhow::anyhow!("dns resolve failed"))?;

        match TcpStream::connect(addr).await {
            Ok(stream) => {
                info!("DirectOutbound connected to {}", target);
                Ok(stream)
            }
            Err(e) => {
                error!("DirectOutbound failed to connect to {}: {}", target, e);

                Err(e).with_context(|| format!("Failed to connect to {}", target))
            }
        }
    }
}
