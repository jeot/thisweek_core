// pub const LANGUAGE_EN: i32 = 1;
// pub const LANGUAGE_FA: i32 = 2;

use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default, PartialEq)]
pub enum Language {
    #[default]
    English = 1,
    Farsi = 2,
    Chinese = 3,
    Arabic = 4,
}

impl Language {
    pub fn change_numbers_language(&self, s: &str) -> String {
        let nums_en = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let nums_fa = ["۰", "۱", "۲", "۳", "۴", "۵", "۶", "۷", "۸", "۹"];
        let nums_ar = ["٠", "١", "٢", "٣", "٤", "٥", "٦", "٧", "٨", "٩"];
        let mut s: String = String::from(s);
        if *self == Language::Farsi {
            for i in 0..10 {
                s = s.replace(nums_en[i], nums_fa[i]);
            }
        } else if *self == Language::Arabic {
            for i in 0..10 {
                s = s.replace(nums_en[i], nums_ar[i]);
            }
        } else {
            // for i in 0..10 {
            //     s = s.replace(nums_fa[i], nums_en[i]);
            // }
        }
        s
    }

    pub fn default_direction(&self) -> String {
        match self {
            Language::English => "ltr".into(),
            Language::Farsi => "rtl".into(),
            Language::Chinese => "ltr".into(),
            Language::Arabic => "rtl".into(),
        }
    }
}

impl From<i32> for Language {
    fn from(val: i32) -> Self {
        match val {
            1 => Language::English,
            2 => Language::Farsi,
            3 => Language::Chinese,
            4 => Language::Arabic,
            _ => Language::English,
        }
    }
}

impl From<String> for Language {
    fn from(val: String) -> Self {
        match val.as_str() {
            "en" => Language::English,
            "fa" => Language::Farsi,
            "cn" => Language::Chinese,
            "ar" => Language::Arabic,
            &_ => Language::English, // default
        }
    }
}

impl From<Language> for String {
    fn from(val: Language) -> Self {
        match val {
            Language::English => "en".to_string(),
            Language::Farsi => "fa".to_string(),
            Language::Chinese => "cn".to_string(),
            Language::Arabic => "ar".to_string(),
        }
    }
}

// convert from constant &str array to Vec
pub fn str_to_vec(arr: &[&str]) -> Vec<String> {
    arr.to_vec().into_iter().map(String::from).collect()
}
