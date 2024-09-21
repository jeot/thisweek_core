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
use ptime;
use serde::Serialize;
use time::Timespec;

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

    fn get_persian_first_and_last_week_days(&self) -> (ptime::Tm, ptime::Tm) {
        let shanbeh = ptime::at(Timespec::new((self.start_day as i64) * 24 * 3600, 0));
        let jomeh = ptime::at(Timespec::new((self.end_day as i64) * 24 * 3600, 0));
        (shanbeh, jomeh)
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
                aux_language.into(),
            )
            .unwrap_or_default()
        });
        self.week_view.items = self.items.iter().map(|i| ItemView::from(i)).collect();
        Ok(())
    }

    pub fn get_view(&self) -> WeekView {
        self.week_view.clone()
    }

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
        self.update()
    }

    pub fn add_new_item(&mut self, kind: i32, text: String) -> AppResult<()> {
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let calendar: i32 = main_cal.into();
        let ordering_key: String = self.get_new_ordering_key();
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

    fn get_ordering_key_of_id(&self, id: i32) -> Result<Option<String>> {
        let pos = self
            .items
            .iter()
            .position(|item| item.id == id)
            .ok_or("invalid ordering key".to_string())?;
        Ok(self
            .items
            .get(pos)
            .ok_or("invalid position".to_string())?
            .order_in_week
            .clone())
    }

    fn new_ordering_finished(&self) {
        let _ = db_sqlite::update_items(&self.items);
    }
}

#[cfg(test)]
mod tests {
    use crate::week::Week;
    use crate::weekdays::{WeekDaysUnixOffset, SEVEN_DAY_WEEK_SIZE};

    fn check_correct_reference_from_persian_dates(
        dates: Vec<ptime::Tm>,
        expected_middle_day: i32,
    ) -> bool {
        println!("----");
        for pt in dates {
            let pt_day = (pt.to_timespec().sec / 3600 / 24) as i32;
            let (s, m, e) = Week::calculate_week_start_middle_end_unix_day(
                pt_day,
                WeekDaysUnixOffset::Sat as i32,
                SEVEN_DAY_WEEK_SIZE,
            );
            let week = Week::default();
            let (first, last) = week.get_persian_first_and_last_week_days();
            let date = pt.to_string("yyyy-MM-dd HH:mm:ss");
            let shanbeh = first.to_string("yyyy-MM-dd HH:mm:ss");
            let jomeh = last.to_string("yyyy-MM-dd HH:mm:ss");
            println!(
                "ptime: {}, start_day: {}, middle_day: {}, end_day: {}, week: {} -> {}",
                date, s, m, e, shanbeh, jomeh
            );
            if expected_middle_day != m {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_find_week_period_with_ptime() {
        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1403, 04 - 1, 22, 23, 22, 11, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 22, 23, 59, 36, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 22, 23, 59, 59, 0).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_persian_dates(pt_vec, 19913));

        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1403, 04 - 1, 23, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 23, 0, 0, 0, 888888).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 23, 0, 0, 1, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 23, 0, 0, 11, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 23, 0, 1, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 24, 12, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 25, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 26, 23, 23, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 27, 23, 23, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 28, 23, 59, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 29, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 29, 23, 59, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 29, 23, 59, 59, 19993294).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_persian_dates(pt_vec, 19920));

        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1403, 04 - 1, 30, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 30, 0, 0, 0, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 30, 0, 0, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 30, 0, 1, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 30, 1, 1, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1403, 04 - 1, 31, 6, 6, 6, 6).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_persian_dates(pt_vec, 19927));
    }
}
