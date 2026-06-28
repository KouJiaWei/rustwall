use super::cache::DnsCache;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// dns域名解析器

pub struct Resolver {
    cache: Arc<RwLock<DnsCache>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(DnsCache::default())),
        }
    }

    pub async fn resolve(&self, host: &str, port: u16) -> Result<Vec<SocketAddr>> {
        let cached = {
            let cache = self.cache.read().await;
            cache.get(host).clone()
        };

        if let Some(addrs) = cached {
            info!("DNS cache Hit {}", host);
            return Ok(addrs);
        }

        info!("DNS Query: {}", host);
        let addrs: Vec<SocketAddr> = tokio::net::lookup_host(format!("{}:{}", host, port))
            .await?
            .collect();

        {
            let mut cache = self.cache.write().await;

            cache.insert(host.to_string(), addrs.clone());

            info!("DNS Cache Size: {}", cache.len())
        }

        Ok(addrs)
    }
}
