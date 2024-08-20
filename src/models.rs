use cuid2;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub const CALENDAR_PERSIAN: i32 = 1;
pub const CALENDAR_GREGORIAN: i32 = 2;

pub const ITEM_KIND_GOAL: i32 = 1;
pub const ITEM_KIND_NOTE: i32 = 2;
pub const ITEM_KIND_EVENT: i32 = 3;
pub const STATUS_UNDONE: i32 = 0;
pub const STATUS_DONE: i32 = 1;

pub const LIST_TYPE_WEEKS: i32 = 1;
pub const LIST_TYPE_OBJECTIVES: i32 = 2;

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

pub trait ItemsList {
    fn get_near_items_id(&self, id: i32) -> (Option<i32>, Option<i32>);
}

impl ItemsList for Vec<Item> {
    fn get_near_items_id(&self, id: i32) -> (Option<i32>, Option<i32>) {
        let mut previous = None;
        let mut next = None;
        let mut iter = self.iter();
        if id < 0 {
            // this case is when nothing is selected.
            // return first and last item's id
            if let Some(item) = iter.next() {
                next = Some(item.id);
            }
            if let Some(item) = iter.last() {
                previous = Some(item.id);
            }
            (previous, next)
        } else {
            let position = iter.position(|i| (i.id == id));
            if let Some(pos) = position {
                if pos > 0 {
                    previous = Some(self[pos - 1].id);
                }
                if pos < (self.len() - 1) {
                    next = Some(self[pos + 1].id);
                }
                (previous, next)
            } else {
                (None, None)
            }
        }
    }
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
    pub fn new(
        calendar: i32,
        year: Option<i32>,
        season: Option<i32>,
        month: Option<i32>,
        day: i32,
        kind: i32,
        text: String,
        ordering_key: String,
        is_resolution: bool,
    ) -> Self {
        NewItem {
            calendar,
            year,
            season,
            month,
            day,
            kind,
            fixed_date: false,
            all_day: false,
            title: if kind == ITEM_KIND_GOAL {
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
            order_in_week: if is_resolution {
                None
            } else {
                Some(ordering_key.clone())
            },
            order_in_resolution: if is_resolution {
                Some(ordering_key.clone())
            } else {
                None
            },
            sync: None,
            uuid: Some(cuid2::create_id()),
        }
    }

    pub fn from(item: &Item) -> NewItem {
        NewItem {
            calendar: item.calendar,
            year: item.year,
            season: item.season,
            month: item.month,
            day: item.day,
            kind: item.kind,
            fixed_date: item.fixed_date,
            all_day: item.all_day,
            title: item.title.clone(),
            note: item.note.clone(),
            datetime: item.datetime.clone(),
            duration: item.duration,
            status: item.status,
            order_in_week: item.order_in_week.clone(),
            order_in_resolution: item.order_in_resolution.clone(),
            sync: None,
            uuid: Some(cuid2::create_id()),
        }
    }

    pub fn new_weekly_goal(calendar: i32, day: i32, text: String, ordering_key: String) -> Self {
        let year = None;
        let season = None;
        let month = None;
        Self::new(
            calendar,
            year,
            season,
            month,
            day,
            ITEM_KIND_GOAL,
            text,
            ordering_key,
            false,
        )
    }

    pub fn new_weekly_note(calendar: i32, day: i32, text: String, ordering_key: String) -> Self {
        let year = None;
        let season = None;
        let month = None;
        Self::new(
            calendar,
            year,
            season,
            month,
            day,
            ITEM_KIND_NOTE,
            text,
            ordering_key,
            false,
        )
    }
}

#[derive(Serialize)]
pub struct FullState {
    today: crate::today::Today,
    week: crate::week::Week,
    year: crate::year::Year,
}
