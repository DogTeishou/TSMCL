use serde::Deserialize;

use crate::Allowed;

#[derive(Deserialize)]
pub struct Processor {
    pub sides: Option<Vec<String>>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
}

impl Allowed for Processor {
    fn allowed(&self) -> bool {
        let mut allowed = false;
        if let Some(sides) = &self.sides {
            for v in sides {
                if v.contains("client") {
                    allowed = true;
                }
            }
        } else {
            allowed = true
        }
        allowed
    }
}