use crate::language::Language;
// use crate::prelude::Error;
use super::DateViewTrait;
use crate::prelude::Result as AppResult;
use crate::week_info::DateView;
use chrono::Datelike;
use chrono::{DateTime, Local};

pub struct GregorianCalendar;

impl DateViewTrait for GregorianCalendar {
    // todo: how to implement language?
    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> AppResult<DateView> {
        let date = DateView {
            day: datetime.day().to_string(),
            month: datetime.month().to_string(),
            weekday: datetime.weekday().to_string(),
            year: datetime.year().to_string(),
        };
        Ok(date)
    }
}
