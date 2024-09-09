/* Week */

// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

// use std::time;

use crate::calendar::Calendar;
use crate::config;
use crate::db_sqlite;
use crate::models::*;
use crate::ordering::Ordering;
use crate::ordering::Result;
use crate::prelude::Result as AppResult;
use crate::today;
use crate::week_info::WeekInfo;
use ptime;
use serde::Serialize;
use time::Timespec;

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

impl Into<WeekDaysUnixOffset> for String {
    fn into(self) -> WeekDaysUnixOffset {
        match self.as_str() {
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

impl Into<WeekDaysUnixOffset> for i32 {
    fn into(self) -> WeekDaysUnixOffset {
        match self {
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

pub const SEVEN_DAY_WEEK_SIZE: i32 = 7;

#[derive(Debug, Serialize, Clone, Default)]
pub struct Week {
    pub title: String,
    pub info: String,
    pub week_info: WeekInfo,
    pub aux_week_info: Option<WeekInfo>,
    pub reference_day: i32,
    pub start_day: i32,
    pub middle_day: i32,
    pub end_day: i32,
    pub items: Vec<Item>,
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
        // todo: seperate updating week_info and week_items... why?
        // update calendar based week informations
        let today = today::get_unix_day();
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let main_cal_lang = config::get_config().main_calendar_language.into();
        self.week_info = WeekInfo::from_unix_start_end_days(
            self.start_day,
            self.end_day,
            today,
            main_cal,
            main_cal_lang,
        )?;
        let aux_cal: Option<Calendar> = config::get_config().secondary_calendar.map(|s| s.into());
        self.aux_week_info = aux_cal.map(|cal| {
            let aux_cal_lang: String = config::get_config()
                .secondary_calendar_language
                .unwrap_or_default();
            WeekInfo::from_unix_start_end_days(
                self.start_day,
                self.end_day,
                today,
                cal,
                aux_cal_lang.into(),
            )
            .unwrap_or_default()
        });
        // todo:
        // self.title = self.week_title();
        self.title = "".into();

        // update items
        let result = db_sqlite::read_items_between_days(self.start_day, self.end_day, true);
        match result {
            Ok(vec) => {
                // todo: exclude the objectives, include the ones that are fixed date
                self.items = vec;
                self.check_and_fix_ordering();
                Ok(())
            }
            Err(e) => Err(e),
        }
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

    // pub fn week_title(&self) -> String {
    //     let today = ptime::now();
    //     let (shanbeh, jomeh) = self.get_persian_first_and_last_week_days();
    //     if shanbeh.tm_year == jomeh.tm_year && shanbeh.tm_year != today.tm_year {
    //         format!(
    //             "{} - {}",
    //             shanbeh.to_string("E d MMM"),
    //             jomeh.to_string("E d MMM، (سال yyyy)")
    //         )
    //     } else if shanbeh.tm_year == jomeh.tm_year && shanbeh.tm_year == today.tm_year {
    //         format!(
    //             "{} - {}",
    //             shanbeh.to_string("E d MMM"),
    //             jomeh.to_string("E d MMM")
    //         )
    //     } else {
    //         format!(
    //             "{} - {}",
    //             shanbeh.to_string("E d MMM yyyy"),
    //             jomeh.to_string("E d MMM yyyy")
    //         )
    //     }
    // }

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

    pub fn backup_database_file(&self) -> Result<()> {
        db_sqlite::backup_database_file()
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
    use crate::week::{Week, WeekDaysUnixOffset, SEVEN_DAY_WEEK_SIZE};

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
