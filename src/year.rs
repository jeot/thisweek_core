use crate::calendar::Calendar;
/* Year */
use crate::config;
use crate::db_sqlite;
use crate::language::Language;
use crate::ordering;
use crate::ordering::Result;
use crate::today;
use crate::{models::*, ordering::Ordering};
use serde::Serialize;

const MAIN_CALENDAR: u32 = 0;
const SECONDARY_CALENDAR: u32 = 1;

#[derive(Debug, Default)]
pub struct Year {
    pub reference_year: i32,
    pub reference_calendar: u32,
    pub calendar: Calendar,
    pub language: Language,
    pub items: Vec<Item>,

    // for view only
    pub year_view: YearView,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct YearView {
    pub year: String,
    pub title: String,
    pub info: String,
    pub items: Vec<ItemView>,
}

impl Year {
    pub fn new() -> Year {
        let mut year = Year {
            reference_calendar: MAIN_CALENDAR,
            ..Default::default()
        };
        let _ = year.current();
        year
    }

    pub fn update(&mut self) -> Result<()> {
        let main_pair = config::get_main_cal_lang_pair();
        let second_pair = config::get_second_cal_lang_pair();
        if self.reference_calendar == SECONDARY_CALENDAR && second_pair.is_some() {
            self.calendar = second_pair.clone().unwrap().calendar;
            self.language = second_pair.unwrap().language;
        } else {
            // MAIN_CALENDAR
            self.calendar = main_pair.calendar;
            self.language = main_pair.language;
        }

        // update items
        let items = db_sqlite::read_items_in_calendar_year(
            self.calendar.clone().into(),
            self.reference_year,
        )?;
        self.items = items;
        self.check_and_fix_ordering();

        // update yearly view
        self.update_year_title_info();
        self.year_view.items = self.items.iter().map(|i| ItemView::from(i)).collect();
        Ok(())
    }

    pub fn get_view(&self) -> YearView {
        self.year_view.clone()
    }

    pub fn get_calendar(&self) -> &Calendar {
        &self.calendar
    }

    fn update_year_title_info(&mut self) {
        self.year_view.year = self.reference_year.to_string();
        self.year_view.year = self.language.change_numbers_language(&self.year_view.year);
        self.year_view.title = match self.language {
            Language::English => format!("Year {}", self.year_view.year),
            Language::Farsi => format!("سال {}", self.year_view.year),
        };
        self.year_view.info = String::new();
    }

    pub fn next(&mut self) -> Result<()> {
        self.reference_year += 1;
        self.update()
    }

    pub fn previous(&mut self) -> Result<()> {
        self.reference_year -= 1;
        self.update()
    }

    pub fn current(&mut self) -> Result<()> {
        self.reference_year = today::get_today_date(&self.calendar).year;
        self.update()
    }

    pub fn switch_calendar(&mut self) -> Result<()> {
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let aux_cal: Option<Calendar> = config::get_config()
            .secondary_calendar_type
            .map(|s| s.into());
        if self.reference_calendar == MAIN_CALENDAR && aux_cal.is_some() {
            self.reference_calendar = SECONDARY_CALENDAR;
            self.calendar = aux_cal.unwrap();
        } else {
            self.reference_calendar = MAIN_CALENDAR;
            self.calendar = main_cal;
        }
        // self.update()
        self.current()
    }

    pub fn move_item_to_other_time_period_offset(&mut self, id: i32, offset: i32) -> Result<usize> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            let mut item = self.items[pos].clone();
            let year = item.year.unwrap_or(self.reference_year) + offset;
            item.year = Some(year);
            item.order_in_resolution = None;
            let result = db_sqlite::update_item(&item);
            let _ = self.update();
            result
        } else {
            let _ = self.update();
            Err("id not in list!".into())
        }
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

    fn new_ordering_finished(&self) {
        let _ = db_sqlite::update_items(&self.items);
    }
}
