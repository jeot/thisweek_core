use crate::language::Language;
use crate::prelude::Result as AppResult;
use crate::week_info::DateView;
use chrono::{DateTime, Local};
use ptime;
use time::Timespec;

use super::DateViewTrait;

pub struct PersianCalendar;

impl DateViewTrait for PersianCalendar {
    // todo: how to implement language?
    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> AppResult<DateView> {
        let ts = datetime.timestamp();
        let pdate = ptime::at(Timespec::new(ts, 0));
        let date = DateView {
            day: pdate.to_string("dd"),
            month: pdate.to_string("MMM"),
            weekday: pdate.to_string("E"),
            year: pdate.to_string("yyyy"),
        };
        Ok(date)
    }
}

/*
    pub fn week_title(&self) -> String {
        let today = ptime::now();
        let (shanbeh, jomeh) = self.get_persian_first_and_last_week_days();
        if shanbeh.tm_year == jomeh.tm_year && shanbeh.tm_year != today.tm_year {
            format!(
                "{} - {}",
                shanbeh.to_string("E d MMM"),
                jomeh.to_string("E d MMM، (سال yyyy)")
            )
        } else if shanbeh.tm_year == jomeh.tm_year && shanbeh.tm_year == today.tm_year {
            format!(
                "{} - {}",
                shanbeh.to_string("E d MMM"),
                jomeh.to_string("E d MMM")
            )
        } else {
            format!(
                "{} - {}",
                shanbeh.to_string("E d MMM yyyy"),
                jomeh.to_string("E d MMM yyyy")
            )
        }
    }
*/
