use std::collections::HashMap;
use serde::Deserialize;

use crate::Allowed;

#[derive(Deserialize)]
pub struct Processor {
    pub sides: Vec<String>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
    pub outputs: HashMap<String, String>,
}

impl Allowed for Processor {
    fn allowed(&self) -> bool {
        let mut allowed = false;
        for v in &self.sides {
            if v.contains("client") {
                allowed = true;
            }
        }
        allowed
    }
}