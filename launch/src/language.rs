#[derive(Clone)]
pub enum Lang {
    ZhCn,
    EnUs,
    JpJa,
}

impl Lang {
    pub fn get_str(&self) -> String {
        match self {
            Lang::ZhCn => { String::from("zh_cn") },
            Lang::EnUs => { String::from("en_us") },
            Lang::JpJa => { String::from("jp_ja") },
        }
    }
}