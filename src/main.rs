use weeks_core::config::{self, Config};

fn main() {
    println!("hello test");
    let c: Config = config::load("C:\\Users\\shk\\weeks.toml".into()).unwrap();
    println!("config: {c:?}");
}
