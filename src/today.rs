/* Today */
use chrono::{DateTime, Datelike, Local};
use ptime;
use serde::Serialize;
use time::Timespec;

#[derive(Serialize)]
pub struct Today {
    today_persian_date: String,
    today_english_date: String,
}

impl Today {
    pub fn get_unix_day() -> i32 {
        let a = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days = (a / 3600 / 24) as i32;
        days
    }
}
