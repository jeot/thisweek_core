use ThisWeek_core::{db_sqlite, week::Week, year::Year};

fn main() {
    let path = "C:\\Users\\shk\\Dropbox\\Online\\weeksapp.db.2024-09-30T10-17-50+03-30.backup";
    let result = db_sqlite::is_correct_db(path);
    println!("path: {path}\nresult: {result}");

    println!();

    let path = "hello.txt";
    let result = db_sqlite::is_correct_db(path);
    println!("path: {path}\nresult: {result}");

    return;
    println!("hello rust");
    let mut w = Week::new();
    let _ = w.next();
    let _ = w.next();
    println!("week info: {:#?}", w.week_view.week_info_main);
    let mut y = Year::new();
    let _ = y.next();
    println!("year : {:#?}", y);
}
