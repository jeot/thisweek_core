use crate::language::Language;
use crate::prelude::Error;
use crate::prelude::Result as AppResult;
use crate::week_info::Date;
use crate::week_info::DateView;
use chrono::{DateTime, Local};
use serde::Serialize;

use self::gregorian::GregorianCalendar;
use self::persian::PersianCalendar;

pub mod gregorian;
pub mod persian;

pub const CALENDAR_GREGORIAN: i32 = 0;
pub const CALENDAR_PERSIAN: i32 = 1;

pub const CALENDAR_PERSIAN_STRING: &str = "Persian";
pub const CALENDAR_GREGORIAN_STRING: &str = "Gregorian";

#[derive(Debug, Serialize, Clone)]
pub enum Calendar {
    Gregorian(gregorian::GregorianCalendar),
    Persian(persian::PersianCalendar),
}

impl Default for Calendar {
    fn default() -> Self {
        Calendar::Gregorian(gregorian::GregorianCalendar)
    }
}

impl Calendar {
    pub fn get_date(&self, day: i32) -> Date {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_date(day),
            Calendar::Persian(_) => PersianCalendar::get_date(day),
        }
    }

    pub fn get_date_view(&self, day: i32, lang: &Language) -> DateView {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_date_view(day, lang),
            Calendar::Persian(_) => PersianCalendar::get_date_view(day, lang),
        }
    }

    pub fn get_calendar_view(&self, lang: &Language) -> CalendarView {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_calendar_view(lang),
            Calendar::Persian(_) => PersianCalendar::get_calendar_view(lang),
        }
    }

    pub fn get_dates_view(
        &self,
        start_day: i32,
        end_day: i32,
        _lang: &Language,
    ) -> AppResult<Vec<DateView>> {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_dates_view(start_day, end_day, _lang),
            Calendar::Persian(_) => PersianCalendar::get_dates_view(start_day, end_day, _lang),
        }
    }

    pub fn into_direction(&self) -> String {
        match self {
            Calendar::Gregorian(_) => "ltr".into(),
            Calendar::Persian(_) => "rtl".into(),
        }
    }

    // todo: this should be specific for each calendar
    pub fn get_date_string(&self, day: i32, lang: &Language) -> String {
        let dw = self.get_date_view(day, lang);
        // persian example: ("E d MMM yyyy")
        // Sat, 17 Jul 2022
        format!("{} {} {} {}", dw.weekday, dw.day, dw.month, dw.year)
    }
}

impl From<Calendar> for i32 {
    fn from(val: Calendar) -> Self {
        match val {
            Calendar::Gregorian(_) => CALENDAR_GREGORIAN,
            Calendar::Persian(_) => CALENDAR_PERSIAN,
        }
    }
}

impl From<Calendar> for String {
    fn from(val: Calendar) -> Self {
        match val {
            Calendar::Gregorian(_) => CALENDAR_GREGORIAN_STRING.to_string(),
            Calendar::Persian(_) => CALENDAR_PERSIAN_STRING.to_string(),
        }
    }
}

// impl Into<Calendar> for i32 {
//     fn into(self) -> Calendar {
//         self.into()
// match self {
//     self
// }
// if self == CALENDAR_PERSIAN {
//     Calendar::Persian(persian::PersianCalendar)
// } else if self == CALENDAR_GREGORIAN {
//     Calendar::Gregorian(gregorian::GregorianCalendar)
// } else {
//     Calendar::Gregorian(gregorian::GregorianCalendar)
// }
//     }
// }

impl From<String> for Calendar {
    fn from(val: String) -> Self {
        if val == "Persian" {
            Calendar::Persian(persian::PersianCalendar)
        } else if val == "Gregorian" {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        } else {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        }
    }
}

#[derive(Serialize)]
pub struct CalendarView {
    pub calendar: i32,
    pub calendar_name: String,
    pub language: String,
    pub direction: String,
    pub months_names: Vec<String>,
    pub seasons_names: Vec<String>,
}

pub trait CalendarSpecificDateView {
    fn new_date(datetime: DateTime<Local>) -> Date;
    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView;
    fn get_calendar_view(lang: &Language) -> CalendarView;

    // maybe we should use something like this:
    // pub fn dateview_from_gregorian_date(g_year: i32, g_month: i32, g_day: i32, lang: Language) ->
    // AppResult<DateView>;

    fn get_date(day: i32) -> Date {
        let sec: i64 = day as i64 * 3600 * 24;
        let nano: u32 = 0;
        let datetime = DateTime::from_timestamp(sec, nano).expect("this should never happen!!");
        let datetime: DateTime<Local> = datetime.into();
        Self::new_date(datetime)
    }

    fn get_date_view(day: i32, lang: &Language) -> DateView {
        let sec: i64 = day as i64 * 24 * 3600;
        let nano: u32 = 0;
        let datetime = DateTime::from_timestamp(sec, nano).expect("this should never happen!!");
        let datetime: DateTime<Local> = datetime.into();
        let mut dateview = Self::new_date_view(datetime, lang);
        dateview.unix_day = day;
        dateview
    }

    fn get_dates_view(start_day: i32, end_day: i32, lang: &Language) -> AppResult<Vec<DateView>> {
        Self::check_days_range(start_day, end_day)?;
        let mut dates: Vec<DateView> = Vec::new();
        // convert days to DateTime
        for i in start_day..=end_day {
            let date = Self::get_date_view(i, lang);
            dates.push(date);
        }
        Ok(dates)
    }

    fn check_days_range(start_day: i32, end_day: i32) -> AppResult<()> {
        if start_day > end_day {
            Err(Error::BadDaysRangeError)
        } else if (end_day - start_day) > 20 {
            Err(Error::LongDaysRangeError(end_day - start_day))
        } else {
            Ok(())
        }
    }
}
