// main.rs

#![warn(dead_code)]

use std::fs::File;
use std::io::{BufReader, Read};

fn load_bin(filename: String) {
    let fp = File::open(filename).unwrap();
    let bin = std::io::BufReader::new(fp).bytes();

}

fn main() {
    println!("Hello, world!");

    load_bin("rom/sample1.nes".to_string());

}
