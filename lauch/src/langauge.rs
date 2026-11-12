use crate::langauge::Lang::{EnUs, JpJa, ZhCn};

#[derive(Clone)]
pub enum Lang {
    ZhCn,
    EnUs,
    JpJa,
}

impl Lang {
    pub fn get_str(&self) -> String {
        match self {
            ZhCn => { String::from("zh_cn") },
            EnUs => { String::from("en_us") },
            JpJa => { String::from("jp_ja") }
            _ => { String::from("en_us") }
        }
    }
}