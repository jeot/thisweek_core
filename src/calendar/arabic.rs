use crate::language::str_to_vec;
use crate::weekdays::convert_weekday;
use crate::{language::Language, week_info::Date, week_info::DateView};
use chrono::Datelike;
use chrono::{DateTime, Local};
use serde::Serialize;

use super::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_ARABIC};

include!("../weekday_names.rs");
include!("../month_names.rs");
include!("../season_names.rs");
include!("./calendar_names.rs");

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ArabicCalendar;

impl CalendarSpecificDateView for ArabicCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let day = datetime.day() as u8;
        let month = datetime.month() as u8;
        let year = datetime.year();
        let weekday = convert_weekday(datetime.weekday()) as u32;
        use icu::calendar::islamic::IslamicCivil;
        let date_iso = icu::calendar::Date::try_new_iso_date(year, month, day)
            .expect("Failed to initialize ISO Date instance.");
        // Conversion into Indian calendar: 1914-08-02.
        let date = date_iso.to_calendar(IslamicCivil);
        let year = date.year().number;
        let month = date.month().ordinal;
        let day = date.day_of_month().0;
        Date {
            calendar: Calendar::Arabic(ArabicCalendar),
            day,
            month,
            weekday,
            year,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let date = Self::new_date(datetime);
        let day = date.day.to_string();
        let day = lang.change_numbers_language(&day);
        let month0 = (date.month - 1) as usize;
        let month = match lang {
            Language::Arabic => ARABIC_MONTH_NAME_AR[month0],
            Language::Farsi => ARABIC_MONTH_NAME_FA[month0],
            _ => ARABIC_MONTH_NAME_EN[month0],
        };
        let month = month.to_string();
        let year = date.year.to_string();
        let year = lang.change_numbers_language(&year);

        let weekday = date.weekday as usize;
        let full_format = match lang {
            Language::Arabic => format!("{}، {} {} {}", WEEKDAY_NAME_FULL_AR[weekday], day, month, year),
            Language::Farsi => format!("{}، {} {} {}", WEEKDAY_NAME_FULL_FA[weekday], day, month, year),
            _ => format!("{}, {} {} {}", WEEKDAY_NAME_FULL_EN[weekday], day, month, year),
        }.to_string();
        let weekday = match lang {
            Language::Arabic => WEEKDAY_NAME_FULL_AR[weekday],
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday],
            _ => WEEKDAY_NAME_HALF_CAP_EN[weekday],
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
            Language::Arabic => str_to_vec(&ARABIC_MONTH_NAME_AR),
            Language::Farsi => str_to_vec(&ARABIC_MONTH_NAME_FA),
            _ => str_to_vec(&ARABIC_MONTH_NAME_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Arabic => str_to_vec(&SEASON_NAME_AR),
            Language::Farsi => str_to_vec(&SEASON_NAME_FA),
            _ => str_to_vec(&SEASON_NAME_EN),
        };
        let calendar_name: String = match lang {
            Language::Arabic => ARABIC_CALENDAR_NAME_AR.into(),
            Language::Farsi => ARABIC_CALENDAR_NAME_FA.into(),
            _ => ARABIC_CALENDAR_NAME_EN.into(),
        };
        CalendarView {
            calendar: CALENDAR_ARABIC,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
