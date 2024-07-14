// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

// use std::time;

use crate::models::*;
use chrono::{DateTime, Datelike, Local};
use ptime;
use serde::Serialize;
use time::Timespec;

use crate::db_sqlite;

#[derive(Serialize)]
pub struct WeekStateJs {
    today_persian_date: String,
    today_english_date: String,
    week_title: String,
    items: Vec<Item>,
}

pub struct WeekState {
    pub start_day: i32,
    pub middle_day: i32,
    pub end_day: i32,
    pub items: Vec<Item>,
}

impl WeekState {
    pub fn new() -> Self {
        // it's local persian date for now!
        let days_tuple = Self::calculate_week_start_middle_end_unix_day(
            Self::get_current_unix_day(),
            WEEKDAY_UNIX_OFFSET_SAT,
            SEVEN_DAY_WEEK_SIZE,
        );
        Self::from_unix_days_tuple(days_tuple)
    }

    fn get_current_unix_day() -> i32 {
        let a = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days = (a / 3600 / 24) as i32;
        days
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
        let db_result = db_sqlite::read_items_between_days(start_day, end_day);
        match db_result {
            Ok(vec) => {
                println!("db read success. some week data.");
                WeekState {
                    start_day,
                    middle_day,
                    end_day,
                    items: vec,
                }
            }
            Err(err) => {
                println!("db read failed. err: {}", err);
                Self {
                    start_day,
                    middle_day,
                    end_day,
                    items: vec![],
                }
            }
        }
    }

    fn persian_first_and_last_week_days(&self) -> (ptime::Tm, ptime::Tm) {
        let shanbeh = ptime::at(Timespec::new((self.start_day as i64) * 24 * 3600, 0));
        let jomeh = ptime::at(Timespec::new((self.end_day as i64) * 24 * 3600, 0));
        (shanbeh, jomeh)
    }

    fn update(&mut self) {
        let db_result = db_sqlite::read_items_between_days(self.start_day, self.end_day);
        match db_result {
            Ok(vec) => self.items = vec,
            Err(err) => println!("db read failed. err: {}", err),
        }
    }

    pub fn next(&mut self) {
        self.start_day += SEVEN_DAY_WEEK_SIZE;
        self.middle_day += SEVEN_DAY_WEEK_SIZE;
        self.end_day += SEVEN_DAY_WEEK_SIZE;
        self.update();
    }

    pub fn previous(&mut self) {
        self.start_day -= SEVEN_DAY_WEEK_SIZE;
        self.middle_day -= SEVEN_DAY_WEEK_SIZE;
        self.end_day -= SEVEN_DAY_WEEK_SIZE;
        self.update();
    }

    pub fn current(&mut self) {
        let (start, middle, end) = Self::calculate_week_start_middle_end_unix_day(
            Self::get_current_unix_day(),
            WEEKDAY_UNIX_OFFSET_SAT,
            SEVEN_DAY_WEEK_SIZE,
        );
        self.start_day = start;
        self.middle_day = middle;
        self.end_day = end;
        self.update();
    }

    pub fn today_persian_date(&self) -> String {
        let today = ptime::now();
        if today.tm_mday == 6 && today.tm_mon == 11 {
            // my birthday
            today.to_string("E d MMM yyyy 🎉")
        } else if today.tm_mday == 1 && today.tm_mon == 0 {
            // new year
            today.to_string("E d MMM yyyy 🎆️")
        } else {
            today.to_string("E d MMM yyyy")
        }
    }

    pub fn today_english_date(&self) -> String {
        let today: DateTime<Local> = Local::now();
        if today.day() == 25 && today.month() == 2 {
            // my birthday
            today.format("%Y-%m-%d 🎉").to_string()
        } else if today.day() == 1 && today.month() == 1 {
            // new year
            today.format("%Y-%m-%d 🎆️").to_string()
        } else {
            today.format("%Y-%m-%d").to_string()
        }
    }

    pub fn week_title(&self) -> String {
        let today = ptime::now();
        let (shanbeh, jomeh) = self.persian_first_and_last_week_days();
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

    pub fn week_state_js_object(&self) -> WeekStateJs {
        WeekStateJs {
            today_persian_date: self.today_persian_date(),
            today_english_date: self.today_english_date(),
            week_title: self.week_title(),
            items: self.items.clone(),
        }
    }

    // pub fn get_item(&self, _id: String) -> Option<Element> {
    //     let item = self
    //         .elements
    //         .iter()
    //         .find(|e| {
    //             if let Element::Goal { id, .. } = e {
    //                 *id == _id
    //             } else if let Element::Note { id, .. } = e {
    //                 *id == _id
    //             } else {
    //                 false
    //             }
    //         })
    //         .cloned();
    //     item
    // }

    pub fn add_new_goal(&mut self, text: String) {
        println!("adding a new goal: {text}");
        let goal = NewItem::new_goal(self.middle_day, text);
        let _result = db_sqlite::create_item(&goal);
        self.update();
    }

    pub fn add_new_note(&mut self, text: String) {
        println!("adding a new note: {text}");
        let note = NewItem::new_note(self.middle_day, text);
        let _result = db_sqlite::create_item(&note);
        self.update();
    }

    pub fn delete_item(&mut self, id: i32) {
        if id < 0 {
            println!("invalid id for delete_item(). ignored. id {id}");
            return;
        }
        println!("delete item (goal/note/event) with id: {id}");
        let _ = db_sqlite::remove_item(id);
        self.update();
    }

    pub fn update_item(&mut self, id: i32, text: String) {
        if id < 0 {
            println!("invalid id for update_item(). ignored. id {id}");
            return;
        }
        println!("edit item id: {}", id);
        if let Ok(mut item) = db_sqlite::get_item(id) {
            if item.kind == ITEM_KIND_GOAL {
                item.title = Some(text.clone());
            }
            if item.kind == ITEM_KIND_NOTE {
                item.note = Some(text.clone());
            }
            let _ = db_sqlite::update_item(&item);
        }
        self.update();
    }

    pub fn toggle_item_state(&mut self, id: i32) {
        if id < 0 {
            println!("invalid id for toggle_item_state(). ignored. id {id}");
            return;
        }
        println!("toggle_item_state: id: {id}");
        let item = db_sqlite::get_item(id);
        println!("item: {item:?}");
        if let Ok(mut item) = item {
            if item.status == Some(STATUS_DONE) {
                item.status = Some(STATUS_UNDONE)
            } else {
                item.status = Some(STATUS_DONE);
            }
            println!("item: {item:?}");
            let update_result = db_sqlite::update_item(&item);
            println!("update_result: {update_result:?}");
        }
        self.update();
    }
}

#[cfg(test)]
mod tests {
    use crate::week::{WeekState, SEVEN_DAY_WEEK_SIZE, WEEKDAY_UNIX_OFFSET_SAT};

    fn check_correct_reference_from_dates(dates: Vec<ptime::Tm>, expected_middle_day: i32) -> bool {
        println!("----");
        for pt in dates {
            let pt_day = (pt.to_timespec().sec / 3600 / 24) as i32;
            let (s, m, e) = WeekState::calculate_week_start_middle_end_unix_day(
                pt_day,
                WEEKDAY_UNIX_OFFSET_SAT,
                SEVEN_DAY_WEEK_SIZE,
            );
            let week = WeekState {
                start_day: s,
                middle_day: m,
                end_day: e,
                items: vec![],
            };
            let (first, last) = week.persian_first_and_last_week_days();
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
