use crate::config;
use crate::models::Item;
use crate::models::NewItem;
use crate::models::{ITEM_KIND_GOAL, ITEM_KIND_NOTE};
use crate::models::{STATUS_DONE, STATUS_UNDONE};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    // let database_url = env::var("WEEKS_DATABASE_URL").expect("WEEKS_DATABASE_URL must be set");
    let database_url = config::get_config().database;
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_item(new_item: &NewItem) -> Result<usize, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    diesel::insert_into(items)
        .values(new_item)
        .execute(conn)
        .map_err(|err| err.to_string())
}

pub fn remove_item(item_id: i32) -> Result<usize, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    diesel::delete(items.filter(id.eq(item_id)))
        .execute(conn)
        .map_err(|err| err.to_string())
}

pub fn update_items(items: &Vec<Item>) -> Result<usize, String> {
    println!("updating all self items in database...");
    let mut count: usize = 0;
    for item in items {
        let result = update_item(item);
        match result {
            Ok(_) => {
                count += 1;
            }
            Err(e) => {
                println!("error! {e}");
            }
        }
    }
    Ok(count)
}

pub fn update_item(item: &Item) -> Result<usize, String> {
    let conn = &mut establish_connection();

    // for test
    // let query = diesel::update(item).set(item);
    // println!(
    //     "{}",
    //     diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query)
    // );

    // https://diesel.rs/guides/all-about-updates.html
    diesel::update(item) // gets id from this object here
        .set(item) // updates all the other fields from this object
        .execute(conn)
        .map_err(|err| err.to_string())
}

pub fn get_item(item_id: i32) -> Result<Item, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    items
        .filter(id.eq(item_id))
        .first(conn)
        .map_err(|e| e.to_string())
}

pub fn read_items_between_days(
    start_day: i32,
    end_day: i32,
    week_order: bool,
    /* for the future: resolution_order: bool */
) -> Result<Vec<Item>, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    if week_order {
        items
            .filter(day.ge(start_day)) // >=
            .filter(day.le(end_day)) // <=
            .order(order_in_week.asc())
            .select(Item::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    } else {
        items
            .filter(day.ge(start_day)) // >=
            .filter(day.le(end_day)) // <=
            .select(Item::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}

pub fn read_items_in_calendar_year(_calendar: i32, _year: i32) -> Result<Vec<Item>, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    items
        .filter(calendar.eq(_calendar))
        .filter(year.eq(Some(_year)))
        .order(order_in_resolution.asc())
        .select(Item::as_select())
        .load(conn)
        .map_err(|e| e.to_string())
}

pub fn backup_database_file() -> Result<(), String> {
    let database_url = env::var("WEEKS_DATABASE_URL").expect("WEEKS_DATABASE_URL must be set");
    // println!("database_url: {database_url}");
    let mut timestamp = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    timestamp = timestamp.replace(':', "-");
    let mut filename = String::from(&database_url);
    filename.push('.');
    filename.push_str(&timestamp);
    filename.push_str(".backup");
    // println!("filename: {filename}");
    std::fs::copy(database_url, filename)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

fn check_valid_id_range(id: i32) -> Result<(), String> {
    if id < 0 {
        let err = format!("invalid id. ignored. id {id}");
        println!("error: {err}");
        Err(err)
    } else {
        Ok(())
    }
}

pub fn update_item_text(id: i32, text: String) -> Result<usize, String> {
    println!("update_item_text: {}", id);
    check_valid_id_range(id)?;
    let mut item = get_item(id)?;
    if item.kind == ITEM_KIND_GOAL {
        item.title = Some(text.clone());
    }
    if item.kind == ITEM_KIND_NOTE {
        item.note = Some(text.clone());
    }
    update_item(&item)
}

pub fn toggle_item_state(id: i32) -> Result<usize, String> {
    println!("toggle_item_state: id: {id}");
    check_valid_id_range(id)?;
    let mut item = get_item(id)?;
    if item.status == Some(STATUS_DONE) {
        item.status = Some(STATUS_UNDONE)
    } else {
        item.status = Some(STATUS_DONE);
    }
    update_item(&item)
}

pub fn update_item_objective_period(
    id: i32,
    year: Option<i32>,
    season: Option<i32>,
    month: Option<i32>,
) -> Result<usize, String> {
    println!("update_item_objective_period: id: {id}, {year:?}, {season:?}, {month:?}");
    check_valid_id_range(id)?;
    let mut item = get_item(id)?;
    item.year = year;
    item.season = season;
    item.month = month;
    update_item(&item)
}

pub fn update_item_week_ordering_key(id: i32, key: String) -> Result<usize, String> {
    println!("update_item_week_ordering_key: id: {id}");
    check_valid_id_range(id)?;
    let mut item = get_item(id)?;
    item.order_in_week = Some(key);
    update_item(&item)
}

pub fn update_item_year_ordering_key(id: i32, key: String) -> Result<usize, String> {
    println!("update_item_year_ordering_key: id: {id}");
    check_valid_id_range(id)?;
    let mut item = get_item(id)?;
    item.order_in_resolution = Some(key);
    update_item(&item)
}
