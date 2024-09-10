/* Today */
use crate::calendar::gregorian::GregorianCalendar;
use crate::calendar::persian::PersianCalendar;
use crate::calendar::Calendar;
use crate::config;
use crate::language::Language;
use crate::prelude::Result as AppResult;
use serde::Serialize;

use crate::week_info::{Date, DateView};

#[derive(Serialize, Clone)]
pub struct Today {
    main_date: Date,
    main_date_view: DateView,
    aux_date_view: Option<DateView>,
    // today_persian_date: String,
    // today_english_date: String,
}

impl Default for Today {
    fn default() -> Self {
        Self::new()
    }
}

impl Today {
    pub fn new() -> Today {
        let main_calendar: Calendar = config::get_config().main_calendar_type.into();
        let main_language: Language = config::get_config().main_calendar_language.into();
        let aux_calendar: Option<Calendar> = config::get_config()
            .secondary_calendar_type
            .map(|cal| cal.into());

        let day = get_unix_day();
        let main_date = get_today_date(&main_calendar);
        let main_date_view = main_calendar.get_date_view(day, &main_language);
        let aux_date_view = aux_calendar.map(|cal| {
            let aux_language: Language = config::get_config()
                .secondary_calendar_language
                .unwrap_or_default()
                .into();
            cal.get_date_view(day, &aux_language)
        });
        Today {
            main_date,
            main_date_view,
            aux_date_view,
        }
    }

    pub fn update(&mut self) -> AppResult<()> {
        *self = Today::new();
        Ok(())
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
