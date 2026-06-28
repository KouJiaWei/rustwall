use anyhow::Result;
use tokio::net::TcpStream;

use crate::session::request::ConnectRequest;

pub struct Socks5Inbound;

impl Socks5Inbound {
    pub async fn handshake(socket: &mut TcpStream) -> Result<()> {
        ConnectRequest::handshake(socket).await
    }

    pub async fn read_connect(socket: &mut TcpStream) -> Result<ConnectRequest> {
        ConnectRequest::read_connect(socket).await
    }

    pub async fn reply_success(socket: &mut TcpStream) -> Result<()> {
        ConnectRequest::reply_success(socket).await
    }
}
