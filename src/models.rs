use crate::calendar::Calendar;
use crate::calendar::CalendarLanguagePair;
use cuid2;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Debug, Serialize, Clone, Default)]
pub struct ItemView {
    pub id: i32,
    pub calendar: i32,
    pub kind: i32,
    pub text: String,
    pub status: bool,
    pub fixed_day_tag: Option<String>,
    pub objective_tag: Option<ObjectiveTag>,
    pub uuid: Option<String>,
}

pub const OBJECTIVE_TYPE_NONE: i32 = 0;
pub const OBJECTIVE_TYPE_MONTHLY: i32 = 1;
pub const OBJECTIVE_TYPE_SEASONAL: i32 = 2;
pub const OBJECTIVE_TYPE_YEARLY: i32 = 3;

#[derive(Debug, Serialize, Clone, Default)]
pub struct ObjectiveTag {
    pub calendar: i32,
    pub text: String,
    pub r#type: i32,
    pub calendar_name: String,
    pub language: String,
    pub year_string: String,
    pub year: i32,
    pub season: Option<usize>,
    pub month: Option<usize>,
}

impl From<&Item> for ItemView {
    fn from(item: &Item) -> Self {
        let text: String = match item.kind {
            ITEM_KIND_GOAL => item.title.clone().unwrap_or_default(),
            ITEM_KIND_NOTE => item.note.clone().unwrap_or_default(),
            _ => "".into(),
        };
        let status = {
            match item.status.unwrap_or_default() {
                0 => false,
                _ => true,
            }
        };
        let fixed_day_tag = {
            if item.day != 0 && item.fixed_date {
                Some("todo: fixed date!".to_string())
            } else {
                None
            }
        };
        let objective_tag = {
            let cal: &Calendar = &item.calendar.into();
            let cal_lang_pair: CalendarLanguagePair = cal.into();
            cal_lang_pair.get_objective_tag(item.year, item.season, item.month)
        };

        ItemView {
            id: item.id,
            calendar: item.calendar,
            kind: item.kind,
            text,
            status,
            fixed_day_tag,
            objective_tag,
            uuid: item.uuid.clone(),
        }
    }
}

pub trait AsItemsList {
    fn get_near_items_id(&self, id: i32) -> (Option<i32>, Option<i32>);
}

impl AsItemsList for Vec<Item> {
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
    ) -> Self {
        let is_objective: bool = if let Some(_) = year { true } else { false };
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
            order_in_week: if is_objective {
                None
            } else {
                Some(ordering_key.clone())
            },
            order_in_resolution: if is_objective {
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
}
