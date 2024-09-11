// pub const LANGUAGE_EN: i32 = 1;
// pub const LANGUAGE_FA: i32 = 2;

use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub enum Language {
    #[default]
    English = 1,
    Farsi = 2,
}

impl Language {
    pub fn change_numbers_to_farsi(s: &str) -> String {
        let nums = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let numsfa = ["۰", "۱", "۲", "۳", "۴", "۵", "۶", "۷", "۸", "۹"];
        let mut s: String = String::from(s);
        for i in 0..10 {
            s = s.replace(nums[i], numsfa[i]);
        }
        s
    }

    pub fn default_direction(&self) -> String {
        match self {
            Language::English => "ltr".into(),
            Language::Farsi => "rtl".into(),
        }
    }
}

impl From<i32> for Language {
    fn from(val: i32) -> Self {
        match val {
            1 => Language::English,
            2 => Language::Farsi,
            _ => Language::English,
        }
    }
}

impl From<String> for Language {
    fn from(val: String) -> Self {
        match val.as_str() {
            "en" => Language::English,
            "fa" => Language::Farsi,
            &_ => Language::English, // default
        }
    }
}

impl From<Language> for String {
    fn from(val: Language) -> Self {
        match val {
            Language::English => "en".to_string(),
            Language::Farsi => "fa".to_string(),
        }
    }
}

// convert from constant &str array to Vec
pub fn str_to_vec(arr: &[&str]) -> Vec<String> {
    arr.to_vec().into_iter().map(String::from).collect()
}
