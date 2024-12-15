use crate::calendar::Calendar;
use crate::config;
use crate::language::Language;
use crate::prelude::Result as AppResult;
use crate::time;
use chrono::Local;
use serde::Serialize;

use crate::week_info::{Date, DateView};

#[derive(Serialize, Clone)]
pub struct Today {
    main_date: Date,
    main_date_view: DateView,
    aux_date_view: Option<DateView>,
}

impl Default for Today {
    fn default() -> Self {
        Self::new()
    }
}

impl Today {
    pub fn new() -> Today {
        let main_calendar: Calendar = config::get_config().main_calendar_type.into();
        let main_language: Language = config::get_config().main_calendar_language.into();
        let aux_calendar: Option<Calendar> = config::get_config()
            .secondary_calendar_type
            .map(|cal| cal.into());
        let today = Local::now();
        let day = time::get_unix_day_from_local_datetime(today);
        let main_date = get_today_date(&main_calendar);
        let main_date_view = main_calendar.get_date_view(day, &main_language);
        let aux_date_view = aux_calendar.map(|cal| {
            let aux_language: Language = config::get_config()
                .secondary_calendar_language
                .unwrap_or_default()
                .into();
            cal.get_date_view(day, &aux_language)
        });
        Today {
            main_date,
            main_date_view,
            aux_date_view,
        }
    }

    pub fn update(&mut self) -> AppResult<()> {
        *self = Today::new();
        Ok(())
    }
}

pub fn get_today_date(calendar: &Calendar) -> Date {
    let today = Local::now();
    let day = time::get_unix_day_from_local_datetime(today);
    calendar.get_date(day)
}

pub fn get_unix_day() -> i32 {
    let today = Local::now();
    time::get_unix_day_from_local_datetime(today)
}
