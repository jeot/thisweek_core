use thisweek_core::{db_sqlite, week::Week, year::Year};

#[allow(unreachable_code)]
/// only for testing. ignore.
fn main() {
    let terms = vec!["shamim", "hello"];
    let result = db_sqlite::search_texts(terms);
    if let Ok(result) = result {
        let result: Vec<_> = result
            .into_iter()
            .map(|i| (i.id, i.title, i.note))
            .collect();
        println!("results:\n{:#?}", result);
        println!("results count: {}", result.len());
    } else {
        println!("result failed!");
    }

    return;
    let path = "weeksapp.db.2024-09-30T10-17-50+03-30.backup";
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
