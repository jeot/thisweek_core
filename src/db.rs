use std::path::Path;

use redb::{Database, Error, ReadableTable, TableDefinition, RedbValue};

const TABLE_I64_GOALS_NOTES: TableDefinition<i64, (Vec<&str>, Vec<&str>)> = TableDefinition::new("weeks");
const db_file: &'static str = "weeks_db.txt";

pub fn write_week(reference: i64, goals: Vec<String>, notes: Vec<String>) -> Result<(), Error> {
    let db = Database::create(Path::new(db_file))?;
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
    let db = Database::create(Path::new(db_file))?;
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

