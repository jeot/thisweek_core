/* Today */

use crate::models::*;
use chrono::{DateTime, Datelike, Local};
use ptime;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Today {
    calendar: i32,
    year: i32,
    month: i32,
    day: i32,
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
        // it's local persian date for now!
        let date = today_date_tupple(CALENDAR_PERSIAN);
        Today {
            calendar: CALENDAR_PERSIAN,
            year: date.0,
            month: date.1,
            day: date.2,
            today_persian_date: today_persian_date(),
            today_english_date: today_english_date(),
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

pub fn get_year(calendar: i32) -> i32 {
    if calendar == CALENDAR_PERSIAN {
        let today = ptime::now();
        today.tm_year
    } else if calendar == CALENDAR_GREGORIAN {
        let today: DateTime<Local> = Local::now();
        today.year()
    } else {
        println!("calendar not implemented yet!");
        0
    }
}

pub fn today_date_tupple(calendar: i32) -> (i32, i32, i32) {
    if calendar == CALENDAR_PERSIAN {
        let today = ptime::now();
        (today.tm_year, today.tm_mon + 1, today.tm_mday)
    } else if calendar == CALENDAR_GREGORIAN {
        let today: DateTime<Local> = Local::now();
        (today.year(), today.month() as i32, today.day() as i32)
    } else {
        println!("calendar not implemented yet!");
        (0, 0, 0)
    }
}

pub fn today_persian_date() -> String {
    let today = ptime::now();
    if today.tm_mday == 6 && today.tm_mon == 11 {
        // my birthday
        today.to_string("E d MMM yyyy 🎉")
    } else if today.tm_mday == 1 && today.tm_mon == 0 {
        // new year
        today.to_string("E d MMM yyyy 🎆️")
    } else {
        today.to_string("E d MMM yyyy")
    }
}

pub fn today_english_date() -> String {
    let today: DateTime<Local> = Local::now();
    if today.day() == 25 && today.month() == 2 {
        // my birthday
        today.format("%Y-%m-%d 🎉").to_string()
    } else if today.day() == 1 && today.month() == 1 {
        // new year
        today.format("%Y-%m-%d 🎆️").to_string()
    } else {
        today.format("%Y-%m-%d").to_string()
    }
}
