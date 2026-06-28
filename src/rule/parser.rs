use super::{action::Action, rule::Rule};

pub struct RuleParser;

impl RuleParser {
    pub fn parse(line: &str) -> Option<Rule> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 {
            return None;
        }

        if parts[0] != "DOMAIN-SUFFIX" {
            return None;
        }

        let action = match parts[2] {
            "DIRECT" => Action::Direct,
            "PROXY" => Action::Proxy,
            _ => return None,
        };

        Some(Rule {
            suffix: parts[1].to_string(),
            action,
        })
    }
}
