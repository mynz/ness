// main.rs

#![warn(dead_code)]

use std::fs::File;
//use std::io::{BufReader, Read};
use std::io::Read;

fn load_bin(filename: String) {
    let fp = File::open(filename).unwrap();
    let mut reader = std::io::BufReader::new(fp);

    let mut bin: Vec<u8> = Vec::with_capacity(1024 * 1024);

    let mut nread;
    let mut header = [0u8; 16];
    nread = reader.read(&mut header).unwrap();
    println!("header: {}, {:?}", nread, header);

    let sig = std::str::from_utf8(&header[0..4]).unwrap();
    assert_eq!(sig, "NES\u{1a}");


    nread = reader.read_to_end(&mut bin).unwrap();
    println!("{:?}", nread);

    for (i, b) in bin.into_iter().enumerate() {
        println!("b: {}P {:?}", i, b);
    }
}

fn main() {
    println!("Hello, world!");

    load_bin("rom/sample1.nes".to_string());
}
