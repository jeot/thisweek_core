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
use crate::today;
use crate::week_info::WeekInfo;
use ptime;
use serde::Serialize;
use time::Timespec;

// January 1, 1970 was Thursday
// Thu, Fri, Sat, Sun, Mon, Tue, Wed
// 0  , 1  , 2  , 3  , 4  , 5  , 6
pub const WEEKDAY_UNIX_OFFSET_THU: i32 = 0;
pub const WEEKDAY_UNIX_OFFSET_FRI: i32 = 1;
pub const WEEKDAY_UNIX_OFFSET_SAT: i32 = 2;
pub const WEEKDAY_UNIX_OFFSET_SUN: i32 = 3;
pub const WEEKDAY_UNIX_OFFSET_MON: i32 = 4;
pub const WEEKDAY_UNIX_OFFSET_TUE: i32 = 5;
pub const WEEKDAY_UNIX_OFFSET_WED: i32 = 6;

pub const SEVEN_DAY_WEEK_SIZE: i32 = 7;

#[derive(Serialize, Clone)]
pub struct Week {
    pub title: String,
    pub info: String,
    pub week_info: WeekInfo,
    pub aux_week_info: Option<WeekInfo>,
    pub start_day: i32,
    pub middle_day: i32,
    pub end_day: i32,
    pub items: Vec<Item>,
}

impl Week {
    pub fn new() -> Self {
        let _main_cal: Calendar = config::get_config().main_calendar.into();
        let _aux_cal: Option<Calendar> = config::get_config().secondary_calendar.map(|s| s.into());
        let days_tuple = Self::calculate_week_start_middle_end_unix_day(
            today::get_unix_day(),
            WEEKDAY_UNIX_OFFSET_SAT,
            SEVEN_DAY_WEEK_SIZE,
        );
        Self::from_unix_days_tuple(days_tuple)
    }

    // January 1, 1970 was Thursday
    // Thu, Fri, Sat, Sun, Mon, Tue, Wed,
    // 0  , 1  , 2  , 3  , 4  , 5  , 6  ,
    // ex: persian 7 day weeks starts from saturday
    // day_offset = WEEKDAY_UNIX_OFFSET_SAT // 2
    // week_size = SEVEN_DAY_WEEK_SIZE // 7
    fn calculate_week_start_middle_end_unix_day(
        day: i32,
        day_offset: i32,
        week_size: i32,
    ) -> (i32, i32, i32) {
        let start = ((day - day_offset) / week_size) * week_size + day_offset;
        let middle = ((day - day_offset) / week_size) * week_size + day_offset + (week_size / 2);
        let end = ((day - day_offset) / week_size) * week_size + day_offset + week_size - 1;
        (start, middle, end)
    }

    fn from_unix_days_tuple((start_day, middle_day, end_day): (i32, i32, i32)) -> Self {
        // it's local persian date for now!
        let mut week = Week {
            start_day,
            middle_day,
            end_day,
            title: "".into(),
            info: "".into(),
            items: Vec::new(),
        };
        let _ = week.update();
        week
    }

    fn get_persian_first_and_last_week_days(&self) -> (ptime::Tm, ptime::Tm) {
        let shanbeh = ptime::at(Timespec::new((self.start_day as i64) * 24 * 3600, 0));
        let jomeh = ptime::at(Timespec::new((self.end_day as i64) * 24 * 3600, 0));
        (shanbeh, jomeh)
    }

    pub fn update(&mut self) -> Result<()> {
        self.title = self.week_title();
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

    pub fn next(&mut self) -> Result<()> {
        self.start_day += SEVEN_DAY_WEEK_SIZE;
        self.middle_day += SEVEN_DAY_WEEK_SIZE;
        self.end_day += SEVEN_DAY_WEEK_SIZE;
        self.update()
    }

    pub fn previous(&mut self) -> Result<()> {
        self.start_day -= SEVEN_DAY_WEEK_SIZE;
        self.middle_day -= SEVEN_DAY_WEEK_SIZE;
        self.end_day -= SEVEN_DAY_WEEK_SIZE;
        self.update()
    }

    pub fn current(&mut self) -> Result<()> {
        let (start, middle, end) = Self::calculate_week_start_middle_end_unix_day(
            today::get_unix_day(),
            WEEKDAY_UNIX_OFFSET_SAT,
            SEVEN_DAY_WEEK_SIZE,
        );
        self.start_day = start;
        self.middle_day = middle;
        self.end_day = end;
        self.update()
    }

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
    use crate::week::{Week, CALENDAR_PERSIAN, SEVEN_DAY_WEEK_SIZE, WEEKDAY_UNIX_OFFSET_SAT};

    fn check_correct_reference_from_dates(dates: Vec<ptime::Tm>, expected_middle_day: i32) -> bool {
        println!("----");
        for pt in dates {
            let pt_day = (pt.to_timespec().sec / 3600 / 24) as i32;
            let (s, m, e) = Week::calculate_week_start_middle_end_unix_day(
                pt_day,
                WEEKDAY_UNIX_OFFSET_SAT,
                SEVEN_DAY_WEEK_SIZE,
            );
            let week = Week {
                calendar: CALENDAR_PERSIAN,
                title: "".to_string(),
                info: "".to_string(),
                start_day: s,
                middle_day: m,
                end_day: e,
                items: vec![],
            };
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
        assert!(check_correct_reference_from_dates(pt_vec, 19913));

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
        assert!(check_correct_reference_from_dates(pt_vec, 19920));

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
        assert!(check_correct_reference_from_dates(pt_vec, 19927));
    }
}
