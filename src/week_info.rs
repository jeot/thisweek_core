use serde::Serialize;
use crate::calendar::CALENDAR_PERSIAN;

#[derive(Serialize, Clone)]
pub struct WeekInfo {
    calendar: i32,
    dates: Vec<DateView>,
    direction: String,
    language: String,
    month_year_info: String,
}

#[derive(Serialize, Clone, Default)]
pub struct DateView {
    pub day: String,
    pub month: String,
    pub weekday: String,
    pub year: String,
}

impl WeekInfo {
    fn from_unix_start_end_days((start_day, end_day): (i32, i32), calendar: i32, language: String, first_day_of_the_week: i32) -> Self {
        let direction = if calendar == CALENDAR_PERSIAN { "rtl" } else { "ltr" };
        let direction = direction.into();

        // Calendar::from_unix_start_end_days((start_day, end_day), calendar);
        let month_year_info = ???;
        WeekInfo {
                calendar,
                dates,
                direction,
                language,
                month_year_info,
        }
    }
}
