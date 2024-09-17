use super::Calendar;
use super::CalendarSpecificDateView;
use super::CalendarView;
use super::CALENDAR_GREGORIAN;
use super::CALENDAR_GREGORIAN_STRING;
use crate::language::str_to_vec;
use crate::language::Language;
use crate::week_info::Date;
use crate::week_info::DateView;
use crate::weekdays::WeekDaysUnixOffset;
use chrono::Datelike;
use chrono::{DateTime, Local};
use serde::Serialize;

include!("../week_names.rs");

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GregorianCalendar;

fn convert_weekday(weekday: chrono::prelude::Weekday) -> WeekDaysUnixOffset {
    match weekday {
        chrono::Weekday::Mon => WeekDaysUnixOffset::Mon,
        chrono::Weekday::Tue => WeekDaysUnixOffset::Tue,
        chrono::Weekday::Wed => WeekDaysUnixOffset::Wed,
        chrono::Weekday::Thu => WeekDaysUnixOffset::Thu,
        chrono::Weekday::Fri => WeekDaysUnixOffset::Fri,
        chrono::Weekday::Sat => WeekDaysUnixOffset::Sat,
        chrono::Weekday::Sun => WeekDaysUnixOffset::Sun,
    }
}

const MONTH_NAME_FULL_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const MONTH_NAME_FULL_FA: [&str; 12] = [
    "ژانویه",
    "فوریه",
    "مارس",
    "آوریل",
    "می",
    "جون",
    "جولای",
    "آگوست",
    "سپتامبر",
    "اکتبر",
    "نوامبر",
    "دسامبر",
];

const SEASON_NAME_FULL_EN: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

const SEASON_NAME_FULL_FA: [&str; 4] = ["بهار", "تابستان", "پاییز", "زمستان"];

impl CalendarSpecificDateView for GregorianCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        Date {
            calendar: Calendar::Gregorian(GregorianCalendar),
            day: datetime.day(),
            month: datetime.month(),
            weekday: convert_weekday(datetime.weekday()) as u32,
            year: datetime.year(),
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let day = match lang {
            Language::English => datetime.day().to_string(),
            Language::Farsi => Language::change_numbers_to_farsi(&datetime.day().to_string()),
            // _ => datetime.day().to_string(),
        };
        let month = datetime.month0() as usize;
        let month = match lang {
            Language::English => MONTH_NAME_FULL_EN[month],
            Language::Farsi => MONTH_NAME_FULL_FA[month],
            // _ => MONTH_NAME_FULL_EN[month],
        };
        let month = month.to_string();
        let weekday = datetime.weekday();
        let weekday: WeekDaysUnixOffset = convert_weekday(weekday);
        let weekday = weekday as usize;
        let weekday = match lang {
            Language::English => WEEKDAY_NAME_HALF_CAP_EN[weekday],
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday],
            // _ => WEEKDAY_NAME_HALF_CAP_EN[weekday as usize],
        };
        let weekday = weekday.to_string();
        let year = match lang {
            Language::English => datetime.year().to_string(),
            Language::Farsi => Language::change_numbers_to_farsi(&datetime.year().to_string()),
            // _ => datetime.year().to_string(),
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
            Language::English => str_to_vec(&MONTH_NAME_FULL_EN),
            Language::Farsi => str_to_vec(&MONTH_NAME_FULL_FA),
        };
        let seasons_names: Vec<String> = match lang {
            Language::English => str_to_vec(&SEASON_NAME_FULL_EN),
            Language::Farsi => str_to_vec(&SEASON_NAME_FULL_FA),
        };
        let calendar_name: String = match lang {
            Language::English => "Gregorian Calendar".into(),
            Language::Farsi => "تقویم میلادی".into(),
        };
        CalendarView {
            calendar: CALENDAR_GREGORIAN,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
