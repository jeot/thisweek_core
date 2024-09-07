use weeks_core::{week::Week, year::Year};

fn main() {
    println!("hello rust");
    let w = Week::new();
    println!("week info: {:#?}", w.week_info);
    // let y = Year::new();
    // println!("year : {:#?}", y);
}
