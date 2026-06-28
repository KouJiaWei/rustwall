use anyhow::Result;
use tokio::net::TcpStream;

use crate::{
    app::context::AppContext,
    outbound::{OutboundType, manager::OutboundManager},
    rule::action::Action,
    session::request::ConnectRequest,
};

pub struct Dispatcher;

impl Dispatcher {
    pub fn new() -> Self {
        Self
    }

    pub async fn dispatch(&self, ctx: &AppContext, req: &ConnectRequest) -> Result<TcpStream> {
        let action = ctx.rule_engine.match_domain(&req.host);

        let outbound = match action {
            Action::Direct => OutboundType::Direct,
            // 后续实现真正的 shadowsockes·
            Action::Proxy => OutboundType::Direct,
        };

        OutboundManager::connect(ctx, outbound, &req.host, req.port).await
    }
}
