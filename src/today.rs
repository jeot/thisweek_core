/* Today */
use crate::calendar::gregorian::GregorianCalendar;
use crate::calendar::persian::PersianCalendar;
use crate::calendar::Calendar;
use crate::config;
use crate::language::Language;
use serde::Serialize;

use crate::week_info::{Date, DateView};

#[derive(Serialize, Clone)]
pub struct Today {
    calendar: Calendar,
    date_view: DateView,
    today_persian_date: String,
    today_english_date: String,
}

impl Default for Today {
    fn default() -> Self {
        Self::new()
    }
}

impl Today {
    pub fn new() -> Today {
        let calendar: Calendar = config::get_config().main_calendar_type.into();
        let language: Language = config::get_config().main_calendar_language.into();
        let day = get_unix_day();
        // let date = calendar.get_date(day);
        let date_view = calendar.get_date_view(day, &language);
        Today {
            calendar,
            date_view,
            today_persian_date: today_persian_date_string(),
            today_english_date: today_english_date_string(),
        }
    }
}

pub fn get_unix_day() -> i32 {
    let a = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (a / 3600 / 24) as i32
}

pub fn get_today_date(calendar: &Calendar) -> Date {
    let day = get_unix_day();
    calendar.get_date(day)
}

pub fn today_persian_date_string() -> String {
    let day = get_unix_day();
    let cal = Calendar::Persian(PersianCalendar);
    let date: Date = cal.get_date(day);
    let mut date_string: String = cal.get_date_string(day, &Language::Farsi);
    if date.day == 6 && date.month == 12 {
        // my birthday
        date_string.push_str(" 🎉");
    } else if date.day == 1 && date.month == 1 {
        // new year
        date_string.push_str(" 🎆️");
    }
    date_string
}

pub fn today_english_date_string() -> String {
    let day = get_unix_day();
    let cal = Calendar::Gregorian(GregorianCalendar);
    let date: Date = cal.get_date(day);
    let mut date_string: String = cal.get_date_string(day, &Language::English);
    if date.day == 1 && date.month == 1 {
        // new year
        date_string.push_str(" 🎆️");
    }
    date_string
}
