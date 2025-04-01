/* Week */

// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

// use std::time;

use crate::calendar::Calendar;
use crate::config;
use crate::db_sqlite;
use crate::language::Language;
use crate::models::*;
use crate::ordering::Ordering;
use crate::ordering::Result;
use crate::prelude::Result as AppResult;
use crate::today;
use crate::week_info::WeekInfo;
use crate::weekdays::WeekDaysUnixOffset;
use crate::weekdays::SEVEN_DAY_WEEK_SIZE;
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct Week {
    pub reference_day: i32,
    pub start_day: i32,
    pub middle_day: i32,
    pub end_day: i32,
    pub items: Vec<Item>,
    // for frontend view only
    pub week_view: WeekView,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct WeekView {
    pub week_info_main: WeekInfo,
    pub week_info_aux: Option<WeekInfo>,
    pub items: Vec<ItemView>,
}

impl Week {
    pub fn new() -> Self {
        let mut week = Week::default();
        let _ = week.current();
        week
    }

    // January 1, 1970 was Thursday
    // Thu, Fri, Sat, Sun, Mon, Tue, Wed,
    // 0  , 1  , 2  , 3  , 4  , 5  , 6  ,
    // ex: persian 7 day weeks starts from saturday
    // day_offset = WEEKDAY_UNIX_OFFSET_SAT // 2
    // week_size = SEVEN_DAY_WEEK_SIZE // 7
    fn calculate_week_start_middle_end_unix_day(
        unix_day: i32,
        day_offset: i32,
        week_size: i32,
    ) -> (i32, i32, i32) {
        let start = ((unix_day - day_offset) / week_size) * week_size + day_offset;
        let middle =
            ((unix_day - day_offset) / week_size) * week_size + day_offset + (week_size / 2);
        let end = ((unix_day - day_offset) / week_size) * week_size + day_offset + week_size - 1;
        (start, middle, end)
    }

    pub fn update(&mut self) -> AppResult<()> {
        // update general week start/middle/end unix days
        let start_week_day: WeekDaysUnixOffset =
            config::get_config().main_calendar_start_weekday.into();
        let start_week_day_offset: i32 = start_week_day as i32;
        let (start_day, middle_day, end_day) = Self::calculate_week_start_middle_end_unix_day(
            self.reference_day,
            start_week_day_offset,
            SEVEN_DAY_WEEK_SIZE,
        );
        self.start_day = start_day;
        self.middle_day = middle_day;
        self.end_day = end_day;

        // update items
        let items = db_sqlite::read_items_between_days(self.start_day, self.end_day, true)?;
        // todo: exclude the objectives, include the ones that are fixed date
        self.items = items;
        self.check_and_fix_ordering();

        // update view items
        let today = today::get_unix_day();
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let main_cal_lang = config::get_config().main_calendar_language.into();
        self.week_view.week_info_main = WeekInfo::from_unix_start_end_days(
            self.start_day,
            self.end_day,
            today,
            main_cal,
            main_cal_lang,
        )?;
        let aux_cal: Option<Calendar> = config::get_config()
            .secondary_calendar_type
            .map(|s| s.into());
        self.week_view.week_info_aux = aux_cal.map(|cal| {
            let aux_language: Language = config::get_config()
                .secondary_calendar_language
                .unwrap_or_default()
                .into();
            WeekInfo::from_unix_start_end_days(
                self.start_day,
                self.end_day,
                today,
                cal,
                aux_language,
            )
            .unwrap_or_default()
        });
        self.week_view.items = self.items.iter().map(ItemView::from).collect();
        Ok(())
    }

    pub fn get_view(&self) -> WeekView {
        self.week_view.clone()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> AppResult<()> {
        self.reference_day += SEVEN_DAY_WEEK_SIZE;
        self.update()
    }

    pub fn previous(&mut self) -> AppResult<()> {
        self.reference_day -= SEVEN_DAY_WEEK_SIZE;
        self.update()
    }

    pub fn current(&mut self) -> AppResult<()> {
        self.reference_day = today::get_unix_day();
        // println!(
        //     "setting week to current date. reference_day: {}",
        //     self.reference_day
        // );
        self.update()
    }

    pub fn add_new_item(
        &mut self,
        kind: i32,
        text: String,
        after_id: Option<i32>,
    ) -> AppResult<i32> {
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let calendar: i32 = main_cal.into();
        let ordering_key: String = self.get_new_ordering_key(after_id);
        let new_item = NewItem::new(
            calendar,
            None, //year,
            None, //season,
            None, //month,
            self.middle_day,
            kind,
            text,
            ordering_key,
        );
        db_sqlite::create_item(&new_item)
    }

    pub fn move_item_to_other_time_period_offset(&mut self, id: i32, offset: i32) -> Result<usize> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            let mut item = self.items[pos].clone();
            item.day += SEVEN_DAY_WEEK_SIZE * offset;
            item.order_in_week = None;
            let result = db_sqlite::update_item(&item);
            let _ = self.update();
            result
        } else {
            let _ = self.update();
            Err("id not in list!".into())
        }
    }
}

impl Ordering for Week {
    fn get_keys(&self) -> Vec<Option<String>> {
        self.items.iter().map(|i| i.order_in_week.clone()).collect()
    }

    // fn get_ordering_key_of_posision(&self, i: usize) -> Result<Option<String>> {
    //     Ok(self.items.get(i).ok_or("invalid position".to_string())?.order_in_week.clone())
    // }

    fn set_ordering_key_of_posision(&mut self, i: usize, key: Option<String>) -> Result<()> {
        self.items
            .get_mut(i)
            .ok_or("invalid pos".to_string())?
            .order_in_week = key;
        Ok(())
    }

    // fn get_posision_of_id(&self, id: i32) -> Result<usize> {
    //     self.items.iter().position(|item| item.id == id)
    // }

    fn get_ordering_key_of_id(&self, id: i32) -> Option<Option<String>> {
        let pos = self.items.iter().position(|item| item.id == id)?;
        Some(self.items.get(pos).unwrap().order_in_week.clone())
    }

    fn new_ordering_finished(&self) {
        let _ = db_sqlite::update_items(&self.items);
    }
}

#[cfg(test)]
mod tests {
    use crate::time;
    use crate::week::Week;
    use crate::weekdays::{WeekDaysUnixOffset, SEVEN_DAY_WEEK_SIZE};
    use chrono::{DateTime, Local};

    #[test]
    fn test_week_middle_day_ref() {
        let gregorian_dates_and_reference = vec![
            // persian 1403-04-22
            ("2024-07-12 23:22:11 +03:30", 19913),
            ("2024-07-12 23:59:36 +03:30", 19913),
            ("2024-07-12 23:59:59 +03:30", 19913),
            // persian 1403-04-23
            ("2024-07-13 00:00:00 +03:30", 19920),
            ("2024-07-13 00:00:01 +03:30", 19920),
            ("2024-07-13 00:00:11 +03:30", 19920),
            ("2024-07-13 01:01:01 +03:30", 19920),
            ("2024-07-13 12:00:00 +03:30", 19920),
            ("2024-07-14 12:00:00 +03:30", 19920),
            ("2024-07-15 00:00:00 +03:30", 19920),
            ("2024-07-16 23:00:00 +03:30", 19920),
            ("2024-07-17 23:23:00 +03:30", 19920),
            ("2024-07-18 23:23:23 +03:30", 19920),
            ("2024-07-18 23:59:23 +03:30", 19920),
            ("2024-07-19 00:00:00 +03:30", 19920),
            ("2024-07-19 02:00:00 +03:30", 19920),
            ("2024-07-19 05:00:00 +03:30", 19920),
            ("2024-07-19 18:00:00 +03:30", 19920),
            ("2024-07-19 23:00:00 +03:30", 19920),
            ("2024-07-19 23:59:59 +03:30", 19920),
            // persian 1403-04-30
            ("2024-07-20 00:00:00 +03:30", 19927),
            ("2024-07-20 00:00:01 +03:30", 19927),
            ("2024-07-20 00:01:01 +03:30", 19927),
            ("2024-07-20 01:00:01 +03:30", 19927),
            ("2024-07-20 03:00:00 +03:30", 19927),
            ("2024-07-20 04:00:00 +03:30", 19927),
        ];
        assert!(check_gregorian_dates_and_reference(
            gregorian_dates_and_reference
        ));
    }

    fn check_gregorian_dates_and_reference(dates_and_ref: Vec<(&str, i32)>) -> bool {
        println!("----");
        for (date_string, expected_middle_day) in dates_and_ref {
            let dt = date_string.parse::<DateTime<Local>>().unwrap();
            let unix_day = time::get_unix_day_from_local_datetime(dt);
            let (s, m, e) = Week::calculate_week_start_middle_end_unix_day(
                unix_day,
                WeekDaysUnixOffset::Sat as i32,
                SEVEN_DAY_WEEK_SIZE,
            );
            println!(
                "date: {}, start_day: {}, middle_day: {}, end_day: {}",
                dt, s, m, e
            );
            if expected_middle_day != m {
                return false;
            }
        }
        true
    }
}
