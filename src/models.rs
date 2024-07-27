use cuid2;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub const CALENDAR_PERSIAN_ID: i32 = 1;
pub const CALENDAR_GREGORIAN_ID: i32 = 2;

pub const SEVEN_DAY_WEEK_SIZE: i32 = 7;

pub const ITEM_KIND_GOAL: i32 = 1;
pub const ITEM_KIND_NOTE: i32 = 2;
pub const ITEM_KIND_EVENT: i32 = 3;
pub const STATUS_UNDONE: i32 = 0;
pub const STATUS_DONE: i32 = 1;

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

#[derive(
    Queryable, Selectable, Identifiable, AsChangeset, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
pub struct Item {
    pub id: i32,
    pub calendar: i32,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub month: Option<i32>,
    pub day: i32,
    pub kind: i32,
    pub fixed_date: bool,
    pub all_day: bool,
    pub title: Option<String>,
    pub note: Option<String>,
    pub datetime: Option<String>,
    pub duration: Option<i32>,
    pub status: Option<i32>,
    pub order_in_week: Option<String>,
    pub order_in_resolution: Option<String>,
    pub sync: Option<i32>,
    pub uuid: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem {
    pub calendar: i32,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub month: Option<i32>,
    pub day: i32,
    pub kind: i32,
    pub fixed_date: bool,
    pub all_day: bool,
    pub title: Option<String>,
    pub note: Option<String>,
    pub datetime: Option<String>,
    pub duration: Option<i32>,
    pub status: Option<i32>,
    pub order_in_week: Option<String>,
    pub order_in_resolution: Option<String>,
    pub sync: Option<i32>,
    pub uuid: Option<String>,
}

impl NewItem {
    fn new(day: i32, kind: i32, text: String, ordering_key: String) -> Self {
        NewItem {
            calendar: CALENDAR_PERSIAN_ID,
            year: None,
            season: None,
            month: None,
            day,
            kind,
            fixed_date: false,
            all_day: false,
            title: if kind != ITEM_KIND_NOTE {
                Some(text.clone())
            } else {
                None
            },
            note: if kind == ITEM_KIND_NOTE {
                Some(text.clone())
            } else {
                None
            },
            datetime: None,
            duration: None,
            status: Some(STATUS_UNDONE),
            order_in_week: Some(ordering_key),
            order_in_resolution: None,
            sync: None,
            uuid: Some(cuid2::create_id()),
        }
    }

    pub fn new_goal(day: i32, text: String, ordering_key: String) -> Self {
        Self::new(day, ITEM_KIND_GOAL, text, ordering_key)
    }

    pub fn new_note(day: i32, text: String, ordering_key: String) -> Self {
        Self::new(day, ITEM_KIND_NOTE, text, ordering_key)
    }
}
