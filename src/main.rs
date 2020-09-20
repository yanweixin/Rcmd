use std::fs::read_to_string;

use crate::data::redis::check_connection;
use crate::util::json::parse_to_result;
use clink::std::abs;
use clink::clib::hello;
use clink::uuid::{uuid_t, uuid_generate, uuid_generate_random};

mod data;
mod util;
mod math;
mod clink;

fn main() {
    init();
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        hello();
        // let mut a = UUID {
        //     a: 0,
        //     b: 0,
        //     c: 0,
        //     d: 0,
        // };
        // uuid_generate(&mut a);
        // println!("{}",a.a);
        let mut a: uuid_t = [0; 16];
        let ptr = &mut a;
        uuid_generate(ptr);
        println!("{:x?}", ptr);
        uuid_generate_random(ptr);
        println!("{:x?}", ptr);
    }
}

// #[link(name = "clib", kind = "dylib")]
// extern {
//     fn hello() -> c_void;
// }

// #[link(name = "uuid", kind = "dylib")]
// extern {
//     fn uuid_generate(out: &mut UUID) -> c_void;
// }

pub struct UUID {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

fn init() {
    // parse app settings
    let mut path = std::env::current_dir().unwrap();
    path.push("resources");
    path.push("appsettings.json");
    let data = read_to_string(path.as_path()).unwrap();
    let json = parse_to_result(&data);

    println!("{}", data);

    println!("{:?}", std::env::current_dir());
}
