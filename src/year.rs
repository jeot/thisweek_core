/* Year */

use crate::db_sqlite;
use crate::models::*;
use chrono::{DateTime, Datelike, Local};
use ptime;
use serde::Serialize;
use time::Timespec;

#[derive(Serialize)]
pub struct Year {
    pub calendar_type: i32,
    pub year: i32,
    pub items: Vec<Item>,
}

impl Ordering for Year {
    fn get_keys(&self) -> Vec<Option<String>> {
        self.items
            .iter()
            .map(|i| i.order_in_resolution.clone())
            .collect()
    }
}
