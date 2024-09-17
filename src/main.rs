use weeks_core::{week::Week, year::Year};

fn main() {
    println!("hello rust");
    let mut w = Week::new();
    let _ = w.next();
    let _ = w.next();
    println!("week info: {:#?}", w.week_view.week_info_main);
    let mut y = Year::new();
    let _ = y.next();
    println!("year : {:#?}", y);
}
