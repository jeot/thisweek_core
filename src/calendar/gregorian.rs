use super::Calendar;
use super::CalendarSpecificDateView;
use super::CalendarView;
use super::CALENDAR_GREGORIAN;
use crate::language::str_to_vec;
use crate::language::Language;
use crate::week_info::Date;
use crate::week_info::DateView;
use crate::weekdays::convert_weekday;
use chrono::Datelike;
use chrono::{DateTime, Local};
use serde::Serialize;

include!("../weekday_names.rs");
include!("../month_names.rs");
include!("../season_names.rs");
include!("./calendar_names.rs");

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GregorianCalendar;

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
        let day = lang.change_numbers_language(&datetime.day().to_string());
        let month = datetime.month0() as usize;
        let month = match lang {
            Language::English => GREGORIAN_MONTH_NAME_EN[month],
            Language::Farsi => GREGORIAN_MONTH_NAME_FA[month],
            Language::Chinese => GREGORIAN_MONTH_NAME_ZH[month],
            Language::Arabic => GREGORIAN_MONTH_NAME_AR[month],
        };
        let month = month.to_string();
        let year = lang.change_numbers_language(&datetime.year().to_string());

        let weekday = datetime.weekday();
        let weekday = convert_weekday(weekday) as usize;
        let full_format = match lang {
            Language::English => format!("{}, {} {} {}", WEEKDAY_NAME_FULL_EN[weekday], day, month, year),
            Language::Farsi => format!("{}، {} {} {}", WEEKDAY_NAME_FULL_FA[weekday], day, month, year),
            Language::Chinese => format!("{}, {} {} {}", WEEKDAY_NAME_FULL_CN[weekday], day, month, year),
            Language::Arabic => format!("{}، {} {} {}", WEEKDAY_NAME_FULL_AR[weekday], day, month, year),
        }.to_string();
        let weekday = match lang {
            Language::English => WEEKDAY_NAME_FULL_EN[weekday],
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday],
            Language::Chinese => WEEKDAY_NAME_FULL_CN[weekday],
            Language::Arabic => WEEKDAY_NAME_FULL_AR[weekday],
        }.to_string();

        DateView {
            unix_day: 0,
            day,
            month,
            weekday,
            year,
            full_format,
        }
    }

    fn get_calendar_view(lang: &Language) -> CalendarView {
        let months_names: Vec<String> = match lang {
            Language::English => str_to_vec(&GREGORIAN_MONTH_NAME_EN),
            Language::Farsi => str_to_vec(&GREGORIAN_MONTH_NAME_FA),
            Language::Chinese => str_to_vec(&GREGORIAN_MONTH_NAME_ZH),
            Language::Arabic => str_to_vec(&GREGORIAN_MONTH_NAME_AR),
        };
        let seasons_names: Vec<String> = match lang {
            Language::English => str_to_vec(&SEASON_NAME_EN),
            Language::Farsi => str_to_vec(&SEASON_NAME_FA),
            Language::Chinese => str_to_vec(&SEASON_NAME_ZH),
            Language::Arabic => str_to_vec(&SEASON_NAME_AR),
        };
        let calendar_name: String = match lang {
            Language::English => GREGORIAN_CALENDAR_NAME_EN.into(),
            Language::Farsi => GREGORIAN_CALENDAR_NAME_FA.into(),
            Language::Chinese => GREGORIAN_CALENDAR_NAME_ZH.into(),
            Language::Arabic => GREGORIAN_CALENDAR_NAME_AR.into(),
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
