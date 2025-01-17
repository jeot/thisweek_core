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
