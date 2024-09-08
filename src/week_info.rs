use crate::{calendar::Calendar, language::Language, prelude::Result as AppResult};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct WeekInfo {
    calendar: Calendar,
    language: Language,
    dates: Vec<DateView>,
    direction: String,
    month_year_info: String,
}

pub struct Date {
    pub calendar: Calendar,
    pub weekday: u32,
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct DateView {
    pub day: String,
    pub month: String,
    pub year: String,
    pub weekday: String,
}

impl WeekInfo {
    pub fn from_unix_start_end_days(
        start_day: i32,
        end_day: i32,
        today: i32,
        calendar: Calendar,
        language: Language,
    ) -> AppResult<Self> {
        let direction = calendar.into_direction();
        let dates = calendar.get_dates_view(start_day, end_day, &language)?;
        let today_view = calendar.get_date_view(today, &language);
        let month_year_info = Self::calculate_month_year_info(&dates, &today_view);
        Ok(WeekInfo {
            calendar,
            dates,
            direction,
            language,
            month_year_info,
        })
    }

    fn calculate_month_year_info(dates: &Vec<DateView>, today: &DateView) -> String {
        let first_day_year = dates.first().unwrap().year.clone();
        let last_day_year = dates.last().unwrap().year.clone();
        let first_day_month = dates.first().unwrap().month.clone();
        let last_day_month = dates.last().unwrap().month.clone();
        let today_year = today.year.clone();
        let today_month = today.month.clone();

        if first_day_month == last_day_month && first_day_year == today_year {
            return format!("{}", today_month);
        }

        if first_day_month != last_day_month
            && first_day_year == today_year
            && last_day_year == today_year
        {
            return format!("{} - {}", first_day_month, last_day_month);
        }

        if first_day_year != last_day_year {
            return format!(
                "{} {} - {} {}",
                first_day_month, first_day_year, last_day_month, last_day_year
            );
        }

        return format!(
            "{} {} - {} {}",
            first_day_month, first_day_year, last_day_month, last_day_year
        );
    }
}
