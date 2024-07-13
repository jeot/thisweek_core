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
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    // note: only some fields are updating here.
    // more fields should be updated in the future based on new features.
    diesel::update(item)
        .set((
            day.eq(item.day),
            fixed_date.eq(item.fixed_date),
            title.eq(item.title.clone()),
            note.eq(item.note.clone()),
            status.eq(item.status),
        ))
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

pub fn read_items_between_days(start_day: i32, end_day: i32) -> Result<Vec<Item>, String> {
    use crate::schema::items::dsl::*;
    let conn = &mut establish_connection();

    items
        .filter(day.ge(start_day)) // >=
        .filter(day.le(end_day)) // <=
        .select(Item::as_select())
        .load(conn)
        .map_err(|e| e.to_string())
}
