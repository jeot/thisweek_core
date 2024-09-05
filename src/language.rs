// pub const LANGUAGE_EN: i32 = 1;
// pub const LANGUAGE_FA: i32 = 2;

pub enum Language {
    English,
    Farsi,
}

impl Into<Language> for i32 {
    fn into(self) -> Language {
        match self {
            1 => Language::English,
            2 => Language::Farsi,
            _ => Language::English,
        }
    }
}
