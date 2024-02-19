use redb::{Database, Error, ReadableTable, TableDefinition, RedbValue};

const TABLE_U64_GOALS_NOTES: TableDefinition<u64, (Vec<&str>, Vec<&str>)> = TableDefinition::new("weeks");
const db_file: &'static str = "~\\.weeks_db";

fn read(ref_seconds: u64) -> Result<Option<(Vec<String>, Vec<String>)>, Error> {

    let db = Database::create(db_file)?;
    let txn = db.begin_read()?;
    let table = txn.open_table(TABLE_U64_GOALS_NOTES)?;
    let result = table.get(ref_seconds)?;
    if let Some(r) = result {
        let (goals, notes) = r.value();
        let goals: Vec<String> = goals.iter().map(|x| String::from(*x)).collect();
        let notes: Vec<String> = notes.iter().map(|x| String::from(*x)).collect();
        Ok(Some((goals, notes)))
    } else {
        Ok(None)
    }
}
