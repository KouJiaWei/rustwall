use anyhow::{Result, bail};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ConnectRequest {
    pub host: String,
    pub port: u16,
}

impl ConnectRequest {
    pub async fn handshake(socket: &mut TcpStream) -> Result<()> {
        let mut header = [0u8; 2];
        socket.read_exact(&mut header).await?;

        let version = header[0];
        let nmethods = header[1];

        info!("Handshake: version={}, nmethods={}", version, nmethods);

        if version != 0x05 {
            bail!("Unsupported SOCKS version: {}", version);
        }

        let mut methods = vec![0u8; nmethods as usize];
        socket.read_exact(&mut methods).await?;
        info!("Client supports methods: {:?}", methods);

        socket.write_all(&[0x05, 0x00]).await?;
        info!("SOCKS5 greeting successful");

        Ok(())
    }

    pub async fn read_connect(socket: &mut TcpStream) -> Result<ConnectRequest> {
        let mut header = [0u8; 4];
        socket.read_exact(&mut header).await?;

        let version = header[0];
        let cmd = header[1];
        let atyp = header[3];

        if version != 0x05 {
            bail!("Invalid SOCKS5 version: {}", version);
        }

        if cmd != 0x01 {
            bail!("Unsupported command: {}", cmd);
        }

        let host = match atyp {
            0x01 => {
                let mut addr = [0u8; 4];
                socket.read_exact(&mut addr).await?;
                format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3])
            }
            0x03 => {
                let mut len = [0u8; 1];
                socket.read_exact(&mut len).await?;

                let mut domain = vec![0u8; len[0] as usize];
                socket.read_exact(&mut domain).await?;

                String::from_utf8(domain)?
            }
            0x04 => {
                bail!("IPv6 is not supported yet");
            }
            _ => {
                bail!("Invalid address type: {}", atyp);
            }
        };

        let mut port_buf = [0u8; 2];
        socket.read_exact(&mut port_buf).await?;

        let port = u16::from_be_bytes(port_buf);
        info!("SOCKS5 CONNECT target: {}:{}", host, port);

        Ok(ConnectRequest { host, port })
    }

    pub async fn reply_success(socket: &mut TcpStream) -> Result<()> {
        socket
            .write_all(&[
                0x05, // version
                0x00, // success
                0x00, // reserved
                0x01, // address type: IPv4
                0x00, 0x00, 0x00, 0x00, // bind address
                0x00, 0x00, // bind port
            ])
            .await?;

        Ok(())
    }
}
