use crate::language::str_to_vec;
use crate::weekdays::WeekDaysUnixOffset;
use crate::{language::Language, week_info::Date, week_info::DateView};
use chrono::{DateTime, Local};
use ptime;
use serde::Serialize;
use time::Timespec;

use crate::calendar::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_PERSIAN};

use crate::calendar::calendar_names::*;
use crate::month_names::*;
use crate::season_names::*;
use crate::weekday_names::*;

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
        let day = lang.change_numbers_language(&day);
        let month = pt.tm_mon as usize;
        let month = match lang {
            Language::Farsi => PERSIAN_MONTH_NAME_FA[month],
            _ => PERSIAN_MONTH_NAME_EN[month],
        };
        let month = month.to_string();
        let year = pt.tm_year.to_string();
        let year = lang.change_numbers_language(&year);

        let weekday = pt.tm_wday;
        let weekday = convert_weekday(weekday) as usize;
        let full_format = match lang {
            Language::Farsi => format!(
                "{}، {} {} {}",
                WEEKDAY_NAME_FULL_FA[weekday], day, month, year
            ),
            _ => format!(
                "{}, {} {} {}",
                WEEKDAY_NAME_FULL_EN[weekday], day, month, year
            ),
        }
        .to_string();
        let weekday = match lang {
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday],
            _ => WEEKDAY_NAME_HALF_CAP_EN[weekday],
        }
        .to_string();
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
            Language::Farsi => str_to_vec(&PERSIAN_MONTH_NAME_FA),
            _ => str_to_vec(&PERSIAN_MONTH_NAME_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Farsi => str_to_vec(&SEASON_NAME_FA),
            _ => str_to_vec(&SEASON_NAME_EN),
        };
        let calendar_name: String = match lang {
            Language::Farsi => PERSIAN_CALENDAR_NAME_FA.into(),
            _ => PERSIAN_CALENDAR_NAME_EN.into(),
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
