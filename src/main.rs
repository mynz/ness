// main.rs

#![warn(dead_code)]

use std::fs::File;
//use std::io::{BufReader, Read};
use std::io::Read;

struct Image {
    bin: Vec<u8>,
}

impl Image {
    fn get_signature(&self) -> [u8; 4] {
        let mut ret = [0u8; 4];
        ret.copy_from_slice(&self.bin[0..4]);
        ret
    }

    fn get_header(&self) -> &[u8] {
        &self.bin[0..16]
    }

    fn get_bytes_of_prg(&self) -> usize {
        self.get_header()[4] as usize * 16 * 1024
    }

    fn get_bytes_of_chr(&self) -> usize {
        self.get_header()[5] as usize * 8 * 1024
    }

    fn get_prg(&self) -> &[u8] {
        let start = 0x10;
        let end = start + self.get_bytes_of_prg();
        &self.bin[start..end]
    }
    fn get_chr(&self) -> &[u8] {
        let start = 0x10 + self.get_bytes_of_prg();
        let end = start + self.get_bytes_of_chr();
        &self.bin[start..end]
    }
}

fn load_image(filename: String) -> Image {
    let fp = File::open(filename).unwrap();
    let mut bin: Vec<u8> = Vec::with_capacity(1024 * 1024);
    let mut reader = std::io::BufReader::new(fp);
    reader.read_to_end(&mut bin).unwrap();
    Image { bin: bin }
}

#[test]
fn test_image() {
    assert!(true);
    let image = load_image("rom/sample1.nes".to_string());
    let sig = image.get_signature();
    assert_eq!(sig, "NES\u{1a}".as_bytes());

    let header = image.get_header();
    assert_eq!(16, header.len());

    assert_eq!(image.get_bytes_of_prg(), image.get_prg().len());
    assert_eq!(image.get_bytes_of_chr(), image.get_prg().len());
}

fn main() {
    println!("Hello, world!");

    let image = load_image("rom/sample1.nes".to_string());

    println!("image size: {}", image.bin.len());
    println!("image header: {:?}", image.get_header());
    println!(
        "image signature: {:?}, prg: {}, chr: {}",
        image.get_signature(),
        image.get_bytes_of_prg(),
        image.get_bytes_of_chr()
    );

    //println!("prg: {:?}", image.get_prg());
    //println!("prg: {:#?}", image.get_prg());
    //println!("prg: {:?}", image.get_chr());
    println!("chr: {:#?}", image.get_chr());
}
