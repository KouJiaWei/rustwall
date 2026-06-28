pub mod direct;
pub mod manager;

#[derive(Debug, Clone)]
pub enum OutboundType {
    Direct,
}
