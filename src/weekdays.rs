use std::fmt::Display;

use crate::config;

pub const SEVEN_DAY_WEEK_SIZE: i32 = 7;

// January 1, 1970 was Thursday
// Thu, Fri, Sat, Sun, Mon, Tue, Wed
// 0  , 1  , 2  , 3  , 4  , 5  , 6
// pub const WEEKDAY_UNIX_OFFSET_THU: i32 = 0;
// pub const WEEKDAY_UNIX_OFFSET_FRI: i32 = 1;
// pub const WEEKDAY_UNIX_OFFSET_SAT: i32 = 2;
// pub const WEEKDAY_UNIX_OFFSET_SUN: i32 = 3;
// pub const WEEKDAY_UNIX_OFFSET_MON: i32 = 4;
// pub const WEEKDAY_UNIX_OFFSET_TUE: i32 = 5;
// pub const WEEKDAY_UNIX_OFFSET_WED: i32 = 6;

#[repr(i32)]
pub enum WeekDaysUnixOffset {
    Thu = 0,
    Fri = 1,
    Sat = 2,
    Sun = 3,
    Mon = 4,
    Tue = 5,
    Wed = 6,
}

impl From<String> for WeekDaysUnixOffset {
    fn from(val: String) -> Self {
        match val.as_str() {
            "THU" => WeekDaysUnixOffset::Thu,
            "FRI" => WeekDaysUnixOffset::Fri,
            "SAT" => WeekDaysUnixOffset::Sat,
            "SUN" => WeekDaysUnixOffset::Sun,
            "MON" => WeekDaysUnixOffset::Mon,
            "TUE" => WeekDaysUnixOffset::Tue,
            "WED" => WeekDaysUnixOffset::Wed,
            s => panic!("invalid weekday string: {s}"),
        }
    }
}

impl Display for WeekDaysUnixOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            WeekDaysUnixOffset::Thu => write!(f, "THU"),
            WeekDaysUnixOffset::Fri => write!(f, "FRI"),
            WeekDaysUnixOffset::Sat => write!(f, "SAT"),
            WeekDaysUnixOffset::Sun => write!(f, "SUN"),
            WeekDaysUnixOffset::Mon => write!(f, "MON"),
            WeekDaysUnixOffset::Tue => write!(f, "TUE"),
            WeekDaysUnixOffset::Wed => write!(f, "WED"),
        };
        Ok(())
    }
}

impl From<WeekDaysUnixOffset> for usize {
    fn from(value: WeekDaysUnixOffset) -> Self {
        match value {
            WeekDaysUnixOffset::Thu => 0,
            WeekDaysUnixOffset::Fri => 1,
            WeekDaysUnixOffset::Sat => 2,
            WeekDaysUnixOffset::Sun => 3,
            WeekDaysUnixOffset::Mon => 4,
            WeekDaysUnixOffset::Tue => 5,
            WeekDaysUnixOffset::Wed => 6,
        }
    }
}

impl From<usize> for WeekDaysUnixOffset {
    fn from(val: usize) -> Self {
        WeekDaysUnixOffset::from(val as i32)
    }
}

impl From<i32> for WeekDaysUnixOffset {
    fn from(val: i32) -> Self {
        match val {
            0 => WeekDaysUnixOffset::Thu,
            1 => WeekDaysUnixOffset::Fri,
            2 => WeekDaysUnixOffset::Sat,
            3 => WeekDaysUnixOffset::Sun,
            4 => WeekDaysUnixOffset::Mon,
            5 => WeekDaysUnixOffset::Tue,
            6 => WeekDaysUnixOffset::Wed,
            _ => WeekDaysUnixOffset::Thu,
        }
    }
}

pub fn convert_weekday(weekday: chrono::prelude::Weekday) -> WeekDaysUnixOffset {
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

pub fn is_weekday_weekend_holiday(weekday: WeekDaysUnixOffset) -> bool {
    let config = config::get_config();
    config.weekend_holidays.contains(&weekday.to_string())
}
