use anyhow::Result;
use std::sync::Arc;

use crate::{
    app::context::AppContext,
    config::Config,
    dispatcher::dispatcher::Dispatcher,
    dns::resolver::Resolver,
    rule::RuleEngine,
};

pub struct AppBuilder;

impl AppBuilder {
    pub fn build(cfg: &Config) -> Result<Arc<AppContext>> {
        let resolver = Arc::new(Resolver::new());
        let rule_engine = Arc::new(RuleEngine::new(
            &cfg.rules
        ));
        let dispatcher = Arc::new(Dispatcher::new());

        Ok(Arc::new(AppContext {
            resolver,
            rule_engine,
            dispatcher,
        }))
    }
}
