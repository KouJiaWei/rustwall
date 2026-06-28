use std::sync::Arc;

use tracing::{error, info};

use crate::app::context::AppContext;
use crate::inbound::socks5::Socks5Inbound;
use crate::transport::relay::Relay;

pub async fn handle_client(
    mut socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    ctx: Arc<AppContext>,
) {
    info!("Client {} connected", addr);

    if let Err(e) = Socks5Inbound::handshake(&mut socket).await {
        error!("SOCKS5 handshake failed: {}", e);
        return;
    }

    let req = match Socks5Inbound::read_connect(&mut socket).await {
        Ok(req) => req,
        Err(e) => {
            error!("CONNECT parse failed: {}", e);
            return;
        }
    };

    info!("SOCKS5 CONNECT ===== {}:{}", req.host, req.port);

    let mut remote = match ctx.dispatcher.dispatch(ctx.as_ref(), &req).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("Remote connection established: {}:{}", req.host, req.port);

    if let Err(e) = Socks5Inbound::reply_success(&mut socket).await {
        error!("Reply failed: {}", e);
    }

    info!("Start relay");

    if let Err(e) = Relay::start(&mut socket, &mut remote).await {
        error!("Relay failed: {}", e);
    }
}
