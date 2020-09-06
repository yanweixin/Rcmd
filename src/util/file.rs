use std::fs::File;
use std::io::Read;
use log::{info, warn,};

// pub fn read_to_string(path: String) -> String {
//     let mut file = File::open(path).unwrap();
//     let mut data = String::new();
//     file.read_to_string(&mut data).unwrap();
//     return data;
// }