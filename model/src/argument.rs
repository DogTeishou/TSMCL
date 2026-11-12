use serde::{Deserialize, Serialize};
use crate::{Allowed, rule::Rules};

#[derive(Deserialize, Serialize)]
pub struct Arguments {
    pub jvm: Vec<Argument>,
    pub game: Vec<Argument>,
    #[serde(rename = "default-user-jvm", skip_serializing_if = "Option::is_none")]
    pub default_user_jvm: Option<Vec<Argument>>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Argument {
    Plain(String),
    Conditional(ConditionalArgument),
}

#[derive(Deserialize, Serialize)]
pub struct ConditionalArgument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
    pub value: ArgumentValue,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    Single(String),
    Multiple(Vec<String>),
}

impl Allowed for ConditionalArgument {
    fn allowed(&self) -> bool {
        let mut allowed = true;
        if let Some(rules) = &self.rules {
            rules.iter().for_each(|rule| if !rule.allowed() { allowed = false });
        }
        allowed
    }
}