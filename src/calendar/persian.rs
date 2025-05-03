use crate::language::str_to_vec;
use crate::weekdays::convert_weekday;
use crate::weekdays::is_weekday_weekend_holiday;
use crate::{language::Language, week_info::Date, week_info::DateView};
use chrono::{DateTime, Datelike, Local};
use serde::Serialize;

use crate::calendar::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_PERSIAN};

use crate::calendar::calendar_names::*;
use crate::month_names::*;
use crate::season_names::*;
use crate::weekday_names::*;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PersianCalendar;

/* fn convert_weekday(weekday: i32) -> WeekDaysUnixOffset {
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
} */

impl CalendarSpecificDateView for PersianCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let (gy, gm, gd) = (datetime.year(), datetime.month(), datetime.day());
        let (year, month, day) = Self::gregorian_to_jalali(gy, gm, gd);
        let weekday = convert_weekday(datetime.weekday()) as u32;
        Date {
            calendar: Calendar::Persian(PersianCalendar),
            day,
            month,
            weekday,
            year,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let (gy, gm, gd) = (datetime.year(), datetime.month(), datetime.day());
        let (year, month, day) = Self::gregorian_to_jalali(gy, gm, gd);
        let day = lang.change_numbers_language(&day.to_string());
        let month = (month - 1) as usize;
        let month = match lang {
            Language::Farsi => PERSIAN_MONTH_NAME_FA[month],
            _ => PERSIAN_MONTH_NAME_EN[month],
        };
        let month = month.to_string();
        let year = lang.change_numbers_language(&year.to_string());
        let weekday = convert_weekday(datetime.weekday()) as usize;
        let weekend_holiday = is_weekday_weekend_holiday(weekday.into());
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
            weekend_holiday,
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

impl PersianCalendar {
    /// note: the chrono-persian calendar (provided link below) has a bug that tries to convert
    /// from persian dates (output of gregorian_to_jalali function) to chrono::NaiveDate, which
    /// internally checks for gregorian dates and fails on some specific persian dates. (like 31th
    /// of 2nd month is available in Ordibehesht, but not in February.)
    /// source: https://jdf.scr.ir
    /// https://docs.rs/chrono-persian/0.1.2/src/chrono_persian/lib.rs.html#1-140;
    fn gregorian_to_jalali(gy: i32, gm: u32, gd: u32) -> (i32, u32, u32) {
        const G_D_M: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let gy2 = if gm > 2 { gy + 1 } else { gy };

        let mut days = 355666 + (365 * gy) + ((gy2 + 3) / 4) - ((gy2 + 99) / 100)
            + ((gy2 + 399) / 400)
            + gd as i32
            + G_D_M[(gm - 1) as usize];

        let mut jy = -1595 + (33 * (days / 12053));
        days %= 12053;
        jy += 4 * (days / 1461);
        days %= 1461;

        if days > 365 {
            jy += (days - 1) / 365;
            days = (days - 1) % 365;
        }

        let jm = if days < 186 {
            1 + (days / 31)
        } else {
            7 + ((days - 186) / 30)
        };

        let jd = if days < 186 {
            1 + (days % 31)
        } else {
            1 + ((days - 186) % 30)
        };

        (jy, jm as u32, jd as u32)
    }
}
