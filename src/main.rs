use weeks_core::config::{self, Config};

fn main() {
    println!("hello test");
    let c: Config = config::get_config();
    println!("config: {c:?}");
    let c: Config = config::get_config();
    println!("config: {c:?}");
    let c: Config = config::get_config();
    println!("config: {c:?}");
}
