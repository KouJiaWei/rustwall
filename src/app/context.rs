use crate::{dispatcher::dispatcher::Dispatcher, dns::resolver::Resolver, rule::RuleEngine};
use std::sync::Arc;

pub struct AppContext {
    pub resolver: Arc<Resolver>,
    pub rule_engine: Arc<RuleEngine>,
    pub dispatcher: Arc<Dispatcher>,
}
