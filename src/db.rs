use std::path::Path;
use crate::week::{WeekState, Element};
use cuid2;

use redb::{Database, Error, ReadableTable, TableDefinition};

// type of weeks elements in DB:
// a vector of:
// type, id, text, boolean, other parameters(string vector)
type TypeOfElementsInDB<'a> = Vec<(i8, &'a str, &'a str, bool, Vec<&'a str>)>;
const TABLE_I64_WEEKS: TableDefinition<i64, TypeOfElementsInDB> = TableDefinition::new("weeks2");
const DB_ELEMTN_VARIANT_GOAL: i8 = 1;
const DB_ELEMTN_VARIANT_NOTE: i8 = 2;


const DB_FILE: &'static str = "weeks_db.txt";

// convert Vec<(i8, &'a str, &'a str, bool, Vec<&'a str>)> to Vec<Elements>
#[allow(non_snake_case)]
fn convert_ElementsInDB_to_VecElement(db_elements: TypeOfElementsInDB) -> Vec<Element> {
    let mut elements: Vec<Element> = Vec::new();

    for db_element in db_elements {
        let (el_variant, id, text, done, _parameters) = db_element;
        let id = String::from(id);
        if !cuid2::is_cuid2(id.clone()) { continue; }
        let text = String::from(text);
        if text.len() == 0 { continue; }
        let element = match el_variant {
            DB_ELEMTN_VARIANT_NOTE => Element::build_note(id, text),
            DB_ELEMTN_VARIANT_GOAL => Element::build_goal(id, text, done),
            x => {
                eprintln!("Warning! element type {} in DB not valid!", x);
                continue;
            },
        };
        elements.push(element);
    }

    elements
}

// convert Vec<Elements> to Vec<(i8, &'a str, &'a str, bool, Vec<&'a str>)>
#[allow(non_snake_case)]
fn convert_VecElement_to_ElementsInDB(elements: &Vec<Element>) -> TypeOfElementsInDB {
    elements.iter().map(|element| {
        match element {
            Element::Goal{id, text, done} => (DB_ELEMTN_VARIANT_GOAL, id.as_str(), text.as_str(), *done, vec![]),
            Element::Note{id, text} => (DB_ELEMTN_VARIANT_NOTE, id.as_str(), text.as_str(), false, vec![]),
        }
    }).collect()
}

pub fn write_week(week: &WeekState) -> Result<(), Error> {
    let reference = week.reference;
    let db = Database::create(Path::new(DB_FILE))?;
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(TABLE_I64_WEEKS)?;
        let db_elements = convert_VecElement_to_ElementsInDB(&week.elements);
        table.insert(reference, db_elements)?;
    }
    txn.commit()?;
    // println!("db write success.");
    Ok(())
}

pub fn read_week(reference: i64) -> Result<Option<WeekState>, Error> {
    let db = Database::create(Path::new(DB_FILE))?;
    let txn = db.begin_read()?;
    let table = txn.open_table(TABLE_I64_WEEKS)?;
    let result = table.get(reference)?;
    if let Some(r) = result {
        let db_elements: TypeOfElementsInDB = r.value();
        let elements: Vec<Element> = convert_ElementsInDB_to_VecElement(db_elements);
        Ok(Some(WeekState { reference, elements }))
    } else {
        Ok(None)
    }
}

/*

const TABLE_I64_GOALS_NOTES: TableDefinition<i64, (Vec<&str>, Vec<&str>)> = TableDefinition::new("weeks");

pub fn write_week(reference: i64, goals: Vec<String>, notes: Vec<String>) -> Result<(), Error> {
    let db = Database::create(Path::new(DB_FILE))?;
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(TABLE_I64_GOALS_NOTES)?;
        let goals: Vec<&str> = goals.iter().map(String::as_str).collect();
        let notes: Vec<&str> = notes.iter().map(String::as_str).collect();
        table.insert(reference, (goals, notes))?;
    }
    txn.commit()?;
    Ok(())
}

pub fn read_week(reference: i64) -> Result<Option<(Vec<String>, Vec<String>)>, Error> {
    let db = Database::create(Path::new(DB_FILE))?;
    let txn = db.begin_read()?;
    let table = txn.open_table(TABLE_I64_GOALS_NOTES)?;
    let result = table.get(reference)?;
    if let Some(r) = result {
        let (goals, notes) = r.value();
        let goals: Vec<String> = goals.iter().map(|x| String::from(*x)).collect();
        let notes: Vec<String> = notes.iter().map(|x| String::from(*x)).collect();
        Ok(Some((goals, notes)))
    } else {
        Ok(None)
    }
}
*/
