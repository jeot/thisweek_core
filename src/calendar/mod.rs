use crate::language::Language;
use crate::prelude::Error;
use crate::prelude::Result as AppResult;
use crate::week_info::DateView;
use chrono::{DateTime, Local};

pub mod gregorian;
pub mod persian;

pub enum Calendar {
    Gregorian(gregorian::GregorianCalendar),
    Persian(persian::PersianCalendar),
}

pub const CALENDAR_PERSIAN: i32 = 1;
pub const CALENDAR_GREGORIAN: i32 = 2;

impl Into<Calendar> for i32 {
    fn into(self) -> Calendar {
        if self == CALENDAR_PERSIAN {
            Calendar::Persian(persian::PersianCalendar)
        } else if self == CALENDAR_PERSIAN {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        } else {
            Calendar::Gregorian(gregorian::GregorianCalendar)
        }
    }
}

pub trait DateViewTrait {
    fn new_date_view(day: DateTime<Local>, lang: &Language) -> AppResult<DateView>;
    // maybe we should use something like this:
    // pub fn dateview_from_gregorian_date(g_year: i32, g_month: i32, g_day: i32, lang: Language) ->
    // AppResult<DateView>;

    fn dates_from(start_day: i32, end_day: i32, language: i32) -> AppResult<Vec<DateView>> {
        let lang: Language = language.into();
        Self::check_days_range(start_day, end_day)?;
        let mut dates: Vec<DateView> = Vec::new();
        // convert days to DateTime
        //
        //
        for i in start_day..end_day {
            let sec: i64 = i as i64 * 3600 * 24;
            let nano: u32 = 0;
            let datetime = DateTime::from_timestamp(sec, nano)
                .ok_or(Error::InvalidTimestampError { sec, nano })?;
            let datetime: DateTime<Local> = datetime.into();
            let date = Self::new_date_view(datetime, &lang)?;
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
