use crate::weekdays::convert_weekday;
use crate::{
    language::{str_to_vec, Language},
    week_info::{Date, DateView},
};

use crate::calendar::calendar_names::*;
use crate::month_names::*;
use crate::season_names::*;
use crate::weekday_names::*;

use super::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_CHINESE};
use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::{DateTime, Datelike, Local};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ChineseCalendar;

impl CalendarSpecificDateView for ChineseCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let year = datetime.year() as u16;
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let solar_date = SolarDate::from_ymd(year, month, day).unwrap();
        let lunisolar_date = LunisolarDate::from_solar_date(solar_date).unwrap();
        let month_number = lunisolar_date.to_lunar_month().to_u8() as u32;
        Date {
            calendar: Calendar::Chinese(ChineseCalendar),
            weekday: convert_weekday(datetime.weekday()) as u32,
            day: lunisolar_date.to_lunar_day() as u32,
            month: month_number,
            year: lunisolar_date.to_lunisolar_year().to_u16() as i32,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let year = datetime.year() as u16;
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let weekday = datetime.weekday();
        let solar_date = SolarDate::from_ymd(year, month, day).unwrap();
        let lunisolar_date = LunisolarDate::from_solar_date(solar_date).unwrap();
        let day = lunisolar_date.to_lunar_day();
        let month = lunisolar_date.to_lunar_month();
        let year = lunisolar_date.to_lunisolar_year();
        let month_index = (lunisolar_date.to_lunar_month().to_u8() - 1) as u32;
        let day = match lang {
            Language::Chinese => day.to_string(),
            _ => day.to_u8().to_string(),
        };
        let month = match lang {
            // format!("{:#}", month), // simplified chinese names
            Language::Chinese => {
                if month.is_leap_month() {
                    format!(
                        "{}{}",
                        CHINESE_LEAP_MONTH_PREFIX_ZH, CHINESE_MONTH_NAME_ZH[month_index as usize]
                    )
                } else {
                    CHINESE_MONTH_NAME_ZH[month_index as usize].to_string()
                }
            }
            _ => {
                if month.is_leap_month() {
                    format!(
                        "{}{}",
                        CHINESE_LEAP_MONTH_PREFIX_EN, CHINESE_MONTH_NAME_EN[month_index as usize]
                    )
                } else {
                    CHINESE_MONTH_NAME_EN[month_index as usize].to_string()
                }
            }
        };
        let year = match lang {
            // Language::Chinese => year.to_string(), // this is not very intuitive!
            Language::Chinese => year.to_u16().to_string(),
            _ => year.to_u16().to_string(),
        };

        let weekday = convert_weekday(weekday) as usize;
        let full_format = match lang {
            Language::Chinese => format!(
                "{}, {} {} {}",
                WEEKDAY_NAME_FULL_CN[weekday], day, month, year
            ),
            _ => format!(
                "{}, {} {} {}",
                WEEKDAY_NAME_FULL_EN[weekday], day, month, year
            ),
        }
        .to_string();
        let weekday = match lang {
            Language::Chinese => WEEKDAY_NAME_FULL_CN[weekday],
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
            Language::Chinese => str_to_vec(&CHINESE_MONTH_NAME_ZH),
            _ => str_to_vec(&CHINESE_MONTH_NAME_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Chinese => str_to_vec(&SEASON_NAME_ZH),
            _ => str_to_vec(&SEASON_NAME_EN),
        };
        let calendar_name: String = match lang {
            // Language::Chinese => "中国农历".into(),
            Language::Chinese => CHINESE_CALENDAR_NAME_ZH.into(),
            _ => CHINESE_CALENDAR_NAME_EN.into(),
        };
        CalendarView {
            calendar: CALENDAR_CHINESE,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
