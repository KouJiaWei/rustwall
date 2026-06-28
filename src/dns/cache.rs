use std::collections::HashMap;
use std::net::SocketAddr;

pub struct DnsCache {
    entries: HashMap<String, Vec<SocketAddr>>,
}

impl DnsCache {
    pub fn get(&self, host: &str) -> Option<Vec<SocketAddr>> {
        self.entries.get(host).cloned()
    }

    pub fn insert(&mut self, host: String, addr: Vec<SocketAddr>) {
        self.entries.insert(host, addr);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}
