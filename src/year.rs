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

#[derive(Serialize, Clone, Debug, Default)]
pub struct Year {
    pub reference_year: i32,
    pub reference_calendar: u32,
    pub calendar: Calendar,
    pub language: Language,
    // pub year: String,
    pub title: String,
    pub info: String,
    pub items: Vec<Item>,
}

impl Year {
    pub fn new() -> Year {
        let mut year = Year::default();
        year.reference_calendar = MAIN_CALENDAR;
        let _ = year.current();
        year
    }

    pub fn update(&mut self) -> Result<()> {
        let main_cal: Calendar = config::get_config().main_calendar_type.into();
        let main_lang = config::get_config().main_calendar_language.into();
        let aux_cal: Option<Calendar> = config::get_config()
            .secondary_calendar_type
            .map(|s| s.into());
        let aux_lang: Language = config::get_config()
            .secondary_calendar_language
            .unwrap_or_default()
            .into();
        if self.reference_calendar == SECONDARY_CALENDAR && aux_cal.is_some() {
            self.calendar = aux_cal.unwrap_or_default();
            self.language = aux_lang;
        } else { // MAIN_CALENDAR
            self.calendar = main_cal;
            self.language = main_lang;
        }
        let db_result = db_sqlite::read_items_in_calendar_year(
            self.calendar.clone().into(),
            self.reference_year,
        );
        self.create_yearly_title();
        match db_result {
            Ok(vec) => {
                self.items = vec;
                self.check_and_fix_ordering();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn create_yearly_title(&mut self) {
        self.title = match self.language {
            Language::Farsi => Language::change_numbers_to_farsi(&format!("سال {}", self.reference_year)),
            Language::English => format!("Year {}", self.reference_year),
        };
        // println!("{}", self.title);
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
