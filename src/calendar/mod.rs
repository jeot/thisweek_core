use crate::config;
use crate::language::Language;
use crate::models::ObjectiveTag;
use crate::models::*;
use crate::prelude::Error;
use crate::prelude::Result as AppResult;
use crate::week_info::Date;
use crate::week_info::DateView;
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct CalendarLanguagePair {
    pub calendar: Calendar,
    pub language: Language,
}

impl CalendarLanguagePair {
    pub fn get_objective_tag(
        &self,
        year: Option<i32>,
        season: Option<i32>,
        month: Option<i32>,
    ) -> Option<ObjectiveTag> {
        if let Some(year) = year {
            let calview = self.calendar.get_calendar_view(&self.language);
            let year_string = year.to_string();
            let year_string = self.language.change_numbers_language(&year_string);
            let (text, r#type) = if let Some(season) = season {
                let season = season - 1;
                let season = calview.seasons_names[season as usize].clone();
                (format!("{season} {year_string}"), OBJECTIVE_TYPE_SEASONAL)
            } else if let Some(month) = month {
                let month = month - 1;
                let month = calview.months_names[month as usize].clone();
                (format!("{month} {year_string}"), OBJECTIVE_TYPE_MONTHLY)
            } else {
                (year_string.clone(), OBJECTIVE_TYPE_YEARLY)
            };
            Some(ObjectiveTag {
                calendar: calview.calendar,
                text,
                r#type,
                calendar_name: calview.calendar_name.clone(),
                language: calview.language.clone(),
                year_string,
                year,
                season: season.map(|i| i as usize),
                month: month.map(|i| i as usize),
            })
        } else {
            None
        }
    }
}

impl From<&Calendar> for CalendarLanguagePair {
    fn from(val: &Calendar) -> Self {
        let main_pair: CalendarLanguagePair = config::get_main_cal_lang_pair();
        let second_pair: Option<CalendarLanguagePair> = config::get_second_cal_lang_pair();
        if *val == main_pair.calendar {
            main_pair
        } else if let Some(pair) = second_pair {
            if *val == pair.calendar {
                pair
            } else {
                CalendarLanguagePair {
                    calendar: val.clone(),
                    language: Language::default(),
                }
            }
        } else {
            CalendarLanguagePair {
                calendar: val.clone(),
                language: Language::default(),
            }
        }
    }
}

pub mod arabic;
pub mod chinese;
pub mod gregorian;
pub mod persian;

use self::arabic::ArabicCalendar;
use self::chinese::ChineseCalendar;
use self::gregorian::GregorianCalendar;
use self::persian::PersianCalendar;

pub const CALENDAR_GREGORIAN: i32 = 0;
pub const CALENDAR_PERSIAN: i32 = 1;
pub const CALENDAR_CHINESE: i32 = 2;
pub const CALENDAR_ARABIC: i32 = 3;

pub const CALENDAR_GREGORIAN_STRING: &str = "Gregorian";
pub const CALENDAR_PERSIAN_STRING: &str = "Persian";
pub const CALENDAR_CHINESE_STRING: &str = "Chinese";
pub const CALENDAR_ARABIC_STRING: &str = "Arabic";

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Calendar {
    Gregorian(gregorian::GregorianCalendar),
    Persian(persian::PersianCalendar),
    Chinese(chinese::ChineseCalendar),
    Arabic(arabic::ArabicCalendar),
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
            Calendar::Chinese(_) => ChineseCalendar::get_date(day),
            Calendar::Arabic(_) => ArabicCalendar::get_date(day),
        }
    }

    pub fn get_date_view(&self, day: i32, lang: &Language) -> DateView {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_date_view(day, lang),
            Calendar::Persian(_) => PersianCalendar::get_date_view(day, lang),
            Calendar::Chinese(_) => ChineseCalendar::get_date_view(day, lang),
            Calendar::Arabic(_) => ArabicCalendar::get_date_view(day, lang),
        }
    }

    pub fn get_calendar_view(&self, lang: &Language) -> CalendarView {
        match self {
            Calendar::Gregorian(_) => GregorianCalendar::get_calendar_view(lang),
            Calendar::Persian(_) => PersianCalendar::get_calendar_view(lang),
            Calendar::Chinese(_) => ChineseCalendar::get_calendar_view(lang),
            Calendar::Arabic(_) => ArabicCalendar::get_calendar_view(lang),
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
            Calendar::Chinese(_) => ChineseCalendar::get_dates_view(start_day, end_day, _lang),
            Calendar::Arabic(_) => ArabicCalendar::get_dates_view(start_day, end_day, _lang),
        }
    }

    pub fn into_direction(&self) -> String {
        match self {
            Calendar::Gregorian(_) => "ltr".into(),
            Calendar::Persian(_) => "rtl".into(),
            Calendar::Chinese(_) => "ltr".into(),
            Calendar::Arabic(_) => "rtl".into(),
        }
    }
}

impl From<Calendar> for i32 {
    fn from(val: Calendar) -> Self {
        match val {
            Calendar::Gregorian(_) => CALENDAR_GREGORIAN,
            Calendar::Persian(_) => CALENDAR_PERSIAN,
            Calendar::Chinese(_) => CALENDAR_CHINESE,
            Calendar::Arabic(_) => CALENDAR_ARABIC,
        }
    }
}

impl From<Calendar> for String {
    fn from(val: Calendar) -> Self {
        match val {
            Calendar::Gregorian(_) => CALENDAR_GREGORIAN_STRING.to_string(),
            Calendar::Persian(_) => CALENDAR_PERSIAN_STRING.to_string(),
            Calendar::Chinese(_) => CALENDAR_CHINESE_STRING.to_string(),
            Calendar::Arabic(_) => CALENDAR_ARABIC_STRING.to_string(),
        }
    }
}

impl From<String> for Calendar {
    fn from(val: String) -> Self {
        if val == "Persian" {
            Calendar::Persian(persian::PersianCalendar)
        } else if val == "Gregorian" {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        } else if val == "Chinese" {
            Calendar::Chinese(chinese::ChineseCalendar)
        } else if val == "Arabic" {
            Calendar::Arabic(arabic::ArabicCalendar)
        } else {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        }
    }
}

impl From<i32> for Calendar {
    fn from(val: i32) -> Self {
        match val {
            CALENDAR_PERSIAN => Calendar::Persian(persian::PersianCalendar),
            CALENDAR_GREGORIAN => Calendar::Gregorian(gregorian::GregorianCalendar),
            CALENDAR_CHINESE => Calendar::Chinese(chinese::ChineseCalendar),
            CALENDAR_ARABIC => Calendar::Arabic(arabic::ArabicCalendar),
            _ => panic!("@from() not a valid calendar number: {}", val),
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
