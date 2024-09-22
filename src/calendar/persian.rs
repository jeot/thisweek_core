use crate::language::str_to_vec;
use crate::weekdays::WeekDaysUnixOffset;
use crate::{language::Language, week_info::Date, week_info::DateView};
use chrono::{DateTime, Local};
use ptime;
use serde::Serialize;
use time::Timespec;

use super::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_PERSIAN};

include!("../week_names.rs");

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PersianCalendar;

fn convert_weekday(weekday: i32) -> WeekDaysUnixOffset {
    // Weekday since Shanbe - [0, 6](<0, 6>). 0 = Shanbeh, ..., 6 = Jomeh.
    match weekday {
        0 => WeekDaysUnixOffset::Sat,
        1 => WeekDaysUnixOffset::Sun,
        2 => WeekDaysUnixOffset::Mon,
        3 => WeekDaysUnixOffset::Tue,
        4 => WeekDaysUnixOffset::Wed,
        5 => WeekDaysUnixOffset::Thu,
        6 => WeekDaysUnixOffset::Fri,
        _ => WeekDaysUnixOffset::Sat,
    }
}

const MONTH_NAME_FULL_EN: [&str; 12] = [
    "Farvardin",
    "Ordibehesht",
    "Khordad",
    "Tir",
    "Mordad",
    "Shahrivar",
    "Mehr",
    "Aban",
    "Azar",
    "Dey",
    "Bahman",
    "Esfand",
];

const MONTH_NAME_FULL_FA: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

const SEASON_NAME_FULL_EN: [&str; 4] = ["Bahaar", "Tabestan", "Paiz", "Zemestan"];

const SEASON_NAME_FULL_FA: [&str; 4] = ["بهار", "تابستان", "پاییز", "زمستان"];

impl CalendarSpecificDateView for PersianCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let ts = datetime.timestamp();
        let pdate = ptime::at(Timespec::new(ts, 0));
        let weekday: i32 = pdate.tm_wday;
        let weekday: WeekDaysUnixOffset = convert_weekday(weekday);
        Date {
            calendar: Calendar::Persian(PersianCalendar),
            day: pdate.tm_mday as u32,
            month: (pdate.tm_mon + 1) as u32,
            weekday: weekday as u32,
            year: pdate.tm_year,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let ts = datetime.timestamp();
        let pt = ptime::at(Timespec::new(ts, 0));

        let day = pt.tm_mday.to_string();
        let day = match lang {
            Language::Farsi => Language::change_numbers_to_farsi(&day),
            _ => day,
        };
        let month = pt.tm_mon as usize;
        let month = match lang {
            Language::Farsi => MONTH_NAME_FULL_FA[month],
            _ => MONTH_NAME_FULL_EN[month],
        };
        let month = month.to_string();
        let weekday = pt.tm_wday;
        let weekday: WeekDaysUnixOffset = convert_weekday(weekday);
        let weekday = match lang {
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday as usize],
            _ => WEEKDAY_NAME_HALF_CAP_EN[weekday as usize],
        };
        let weekday = weekday.to_string();
        let year = pt.tm_year;
        let year = match lang {
            Language::Farsi => Language::change_numbers_to_farsi(&year.to_string()),
            _ => year.to_string(),
        };

        DateView {
            unix_day: 0,
            day,
            month,
            weekday,
            year,
        }
    }

    fn get_calendar_view(lang: &Language) -> CalendarView {
        let months_names: Vec<String> = match lang {
            Language::Farsi => str_to_vec(&MONTH_NAME_FULL_FA),
            _ => str_to_vec(&MONTH_NAME_FULL_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Farsi => str_to_vec(&SEASON_NAME_FULL_FA),
            _ => str_to_vec(&SEASON_NAME_FULL_EN),
        };
        let calendar_name: String = match lang {
            Language::Farsi => "تقویم شمسی هجری".into(),
            _ => "Persian Calendar".into(),
        };
        CalendarView {
            calendar: CALENDAR_PERSIAN,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
