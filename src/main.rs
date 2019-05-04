// main.rs

#![allow(dead_code)]

use std::fs::File;
use std::fs::OpenOptions;
//use std::io::{BufReader, Read};
use std::io::prelude::*;
use std::io::{Cursor, Read};
use std::path::Path;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

extern crate image;

struct Rom {
    bin: Vec<u8>,
}

impl Rom {
    fn load_image(filename: String) -> Rom {
        let fp = File::open(filename).unwrap();
        let mut bin: Vec<u8> = Vec::with_capacity(1024 * 1024);
        let mut reader = std::io::BufReader::new(fp);
        reader.read_to_end(&mut bin).unwrap();
        Rom { bin: bin }
    }

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

    fn write_png(&self, path: &Path) {
        let chr = self.get_chr();

        let nblocks = chr.len() / 16;
        let w = 8 * 64; // 512
        let h = (nblocks / 64) * 8;

        println!(
            "write_png: {}, nblocks: {}, w,h: {}, {}",
            chr.len(),
            nblocks,
            w,
            h
        );

        let nbuf: usize = w * h * 4;
        let mut buf: Vec<u8> = vec![0u8; nbuf];

        // 一ブロックは 16 byte
        for (idx_block, chunk) in chr.chunks(16).enumerate() {
            let xb = idx_block % 64;
            let yb = idx_block / 64;
            let x_base = xb * 8;
            let y_base = yb * 8;

            for iy in 0..8 {
                for ix in 0..8 {
                    let lines = (chunk[iy], chunk[iy + 8]);
                    let rx = 7 - ix; // reverse endian
                    let a = lines.0 >> rx & 0x1;
                    let b = lines.1 >> rx & 0x1;
                    let bit = a | (b << 1);

                    assert!(bit < 4);
                    let c = match bit {
                        0 => 0x00,
                        1 => 0x55,
                        2 => 0xaa,
                        3 => 0xff,
                        _ => 0xff,
                    };

                    let y = iy + y_base;
                    let x = ix + x_base;
                    let dst_idx = y * w * 4 + x * 4;
                    buf[dst_idx + 0] = c;
                    buf[dst_idx + 1] = c;
                    buf[dst_idx + 2] = c;
                    buf[dst_idx + 3] = 0xff;
                }
            }
        }

        image::save_buffer(path, &buf, w as u32, h as u32, image::RGBA(8)).unwrap();
    }
}

#[test]
fn test_image() {
    assert!(true);
    let rom = Rom::load_image("rom/sample1.nes".to_string());
    let sig = rom.get_signature();
    assert_eq!(sig, "NES\u{1a}".as_bytes());

    let header = rom.get_header();
    assert_eq!(16, header.len());

    assert_eq!(rom.get_bytes_of_prg(), rom.get_prg().len());
    assert_eq!(rom.get_bytes_of_chr(), rom.get_chr().len());
}

struct StatusRegister {
    negative: bool,
    overflow: bool,
    reserved: bool,
    brk: bool, // break
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

impl Default for StatusRegister {
    fn default() -> Self {
        StatusRegister {
            negative: false,
            overflow: false,
            reserved: true,
            brk: true, // break
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }
}

#[derive(Default)]
struct Register {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: StatusRegister,
    pc: u16,
}

impl Register {}

struct Machine {
    register: Register,
    rom: Rom,
    wram: Box<[u8]>, // 2kb
}

//struct Bus { }

impl Machine {
    fn new(rom: Rom) -> Machine {
        let register = Register::default();
        let wram = Box::new([0;2 * 1024]);
        Machine { register, rom, wram }
    }

    fn read_word(&self, addr: u16) -> u16 {
        if addr <= 0x07ff {
            // FIXME, TODO: WRAM領域(2kb)
            let mut c = Cursor::new(&self.wram);
            c.set_position(addr as u64);
            return c.read_u16::<LittleEndian>().unwrap();
        }

        // 0xffff まで有効
        if addr >= 0x8000 {
            let mut cur = Cursor::new(self.rom.get_prg());
            cur.set_position((addr - 0x8000) as u64);
            return cur.read_u16::<LittleEndian>().unwrap();
        }

        return 0;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        let word = self.read_word(addr);
        (word >> 8 & 0x00ff) as u8
    }

    fn reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.read_word(0xfffc);
    }

    fn decode(&mut self) {
        let op = self.read_byte(self.register.pc);
        self.register.pc += 1;

        println!("XXX op: {:x}", op);
    }

    fn run(&mut self) {
        self.reset();

        self.decode();

        // TODO
        //self.register.pc += 1;
        //self.read_byte(self.Register.pc);
    }
}

#[test]
fn test_machine() {
    let rom = Rom::load_image("rom/sample1.nes".to_string());
    let mut machine = Machine::new(rom);

    println!("XXX: {:x?}", machine.read_word(0x8000));

    assert_eq!(0x00, machine.read_word(0));
    assert_eq!(0x00, machine.read_word(0));
    assert_eq!(0x00, machine.read_word(2));
    assert_eq!(0x8000, machine.read_word(0xfffc)); // reset
    assert_eq!(0xa278, machine.read_word(0x8000)); // prg
    assert_eq!(0x9aff, machine.read_word(0x8002)); // prg
    assert_eq!(0x00a9, machine.read_word(0x8004)); // prg
    assert_eq!(0x80, machine.read_byte(0xfffc)); // reset
    assert_eq!(0x00, machine.read_byte(0xfffd)); // reset

    machine.run()
}

fn dump_bin(path: &Path, bin: &[u8]) -> std::io::Result<()> {
    let mut fp = OpenOptions::new().create(true).write(true).open(path)?;
    fp.write_all(bin)?;
    Ok(())
}

fn main() {
    //println!("Hello, world!");

    let rom = Rom::load_image("rom/sample1.nes".to_string());

    println!("rom size: {}", rom.bin.len());
    println!("rom header: {:?}", rom.get_header());
    println!(
        "rom signature: {:?}, prg: {}(0x{:x?}), chr: {}(0x{:x?})",
        rom.get_signature(),
        rom.get_bytes_of_prg(),
        rom.get_bytes_of_prg(),
        rom.get_bytes_of_chr(),
        rom.get_bytes_of_chr()
    );

    //println!("prg: {:?}", rom.get_prg());
    //println!("prg: {:#?}", rom.get_prg());
    //println!("prg: {:?}", rom.get_chr());
    //println!("chr: {:?}", rom.get_chr());

    rom.write_png(&Path::new("tmp/image.png"));
    dump_bin(&Path::new("tmp/prg.bin"), rom.get_prg()).unwrap();

    let mut machine = Machine::new(rom);
    machine.run();
}
