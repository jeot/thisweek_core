use crate::models::Item;
use crate::models::NewItem;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("WEEKS_DATABASE_URL").expect("WEEKS_DATABASE_URL must be set");
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

pub fn backup_database_file() -> Result<usize, String> {
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
        .map(|x| x as usize)
        .map_err(|e| e.to_string())
}
