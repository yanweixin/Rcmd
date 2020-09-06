mod data;
mod util;
mod math;

use std::fs::read_to_string;
use crate::data::redis::check_connection;
use crate::util::json::parse_to_result;

fn main() {
    init();
}

fn init() {
    // parse app settings
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("resources");
    path.push("appsettings.json");
    let data = read_to_string(path.as_path()).unwrap();
    let json = parse_to_result(&data);

    println!("{}", data);

    println!("{:?}", std::env::current_dir());
}
