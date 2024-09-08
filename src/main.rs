use weeks_core::{week::Week, year::Year};

fn main() {
    println!("hello rust");
    let mut w = Week::new();
    w.next();
    w.next();
    println!("week info: {:#?}", w.week_info);
    let mut y = Year::new();
    y.next();
    println!("year : {:#?}", y);
}
