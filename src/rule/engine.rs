use super::{action::Action, parser::RuleParser, rule::Rule};
use tracing::info;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(lines: &[String]) -> Self {
        let mut rules = Vec::new();

        for line in lines {
            match RuleParser::parse(line) {
                Some(rule) => {
                    info!("Load rule: {:?}", rule);
                    rules.push(rule);
                }
                None => {
                    info!("Invalid rule: {}", line);
                }
            }
        }

        Self { rules }
    }

    pub fn match_domain(&self, host: &str) -> Action {
        for rule in &self.rules {
            if host.ends_with(&rule.suffix) {
                return rule.action.clone();
            }
        }
        Action::Direct
    }
}
