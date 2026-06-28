use super::action::Action;

#[derive(Debug, Clone)]
pub struct Rule {
    pub suffix: String,
    pub action: Action,
}
