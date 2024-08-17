/* Year */

use crate::db_sqlite;
use crate::ordering;
use crate::ordering::Result;
use crate::today;
use crate::{models::*, ordering::Ordering};
use chrono::format::format;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Year {
    pub title: String,
    pub info: String,
    pub calendar: i32,
    pub year: i32,
    pub items: Vec<Item>,
}

// impl Default for Year {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Year {
    pub fn new() -> Year {
        // it's local persian year for now!
        let year: i32 = today::get_year(CALENDAR_PERSIAN);
        Self::from_calendar_and_year(CALENDAR_PERSIAN, year)
    }

    pub fn from_calendar_and_year(calendar: i32, year_number: i32) -> Year {
        let mut year = Year {
            title: String::new(),
            info: String::new(),
            calendar,
            year: year_number,
            items: Vec::new(),
        };
        let _ = year.update();
        year
    }

    pub fn update(&mut self) -> Result<()> {
        let db_result = db_sqlite::read_items_in_calendar_year(self.calendar, self.year);
        self.title = format!("سال {}", self.year);
        match db_result {
            Ok(vec) => {
                // println!("read year ok: items: {vec:?}");
                self.items = vec;
                self.check_and_fix_ordering();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn next(&mut self) -> Result<()> {
        self.year += 1;
        self.update()
    }

    pub fn previous(&mut self) -> Result<()> {
        self.year -= 1;
        self.update()
    }

    pub fn current(&mut self) -> Result<()> {
        let year: i32 = today::get_year(CALENDAR_PERSIAN);
        self.year = year;
        self.update()
    }

    pub fn add_new_goal(&mut self, text: String) -> Result<usize> {
        println!("adding a new yearly goal: {text}");
        let ordering_key = self.get_new_ordering_key();
        let goal = NewItem::new(
            self.calendar,
            Some(self.year),
            None,
            None,
            0,
            ITEM_KIND_GOAL,
            text,
            ordering_key,
            true,
        );
        let result = db_sqlite::create_item(&goal);
        let _ = self.update();
        result
    }

    pub fn add_new_note(&mut self, text: String) -> Result<usize> {
        println!("adding a new yearly note: {text}");
        let ordering_key = self.get_new_ordering_key();
        let note = NewItem::new(
            self.calendar,
            Some(self.year),
            None,
            None,
            0,
            ITEM_KIND_NOTE,
            text,
            ordering_key,
            true,
        );
        let result = db_sqlite::create_item(&note);
        let _ = self.update();
        result
    }
}

impl Ordering for Year {
    fn get_keys(&self) -> Vec<Option<String>> {
        self.items
            .iter()
            .map(|i| i.order_in_resolution.clone())
            .collect()
    }

    // fn get_ordering_key_of_posision(&self, i: usize) -> Result<Option<String>> {
    //     Ok(self.items.get(i)?.order_in_resolution.clone())
    // }

    fn set_ordering_key_of_posision(&mut self, i: usize, key: Option<String>) -> Result<()> {
        self.items
            .get_mut(i)
            .ok_or("invalid pos".to_string())?
            .order_in_resolution = key;
        Ok(())
    }

    // fn get_posision_of_id(&self, id: i32) -> Result<usize> {
    //     self.items.iter().position(|item| item.id == id)
    // }

    fn get_ordering_key_of_id(&self, id: i32) -> ordering::Result<Option<String>> {
        let pos = self
            .items
            .iter()
            .position(|item| item.id == id)
            .ok_or("invalid ordering key".to_string())?;
        Ok(self
            .items
            .get(pos)
            .ok_or("invalid position".to_string())?
            .order_in_resolution
            .clone())
    }
}
