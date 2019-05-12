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

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
}

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
    let rom = Rom::load_image("rom/sample1/sample1.nes".to_string());
    let sig = rom.get_signature();
    assert_eq!(sig, "NES\u{1a}".as_bytes());

    let header = rom.get_header();
    assert_eq!(16, header.len());

    assert_eq!(rom.get_bytes_of_prg(), rom.get_prg().len());
    assert_eq!(rom.get_bytes_of_chr(), rom.get_chr().len());
}

enum AddrMode {
    Implied,
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Relative(u8),
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
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
    a: u8, // accumelater
    x: u8, // index register
    y: u8, // index register
    s: u8, // stack pointer
    p: StatusRegister,
    pc: u16,
}

impl Register {}

//// PPU

#[derive(Default)]
struct PpuRegister {
    ctrl: u8,     // w
    mask: u8,     // w
    status: u8,   // r
    oamaddr: u8,  // w
    oamdata: u8,  // rw
    scroll: u8,   // w
    ppuaddr: u16, // w
    ppudata: u8,  // rw

    // ppuaddr の2会書き込み用のトグルフラグ
    toggle_ppuaddr: bool,
}

impl PpuRegister {}

struct PpuUnit {
    register: PpuRegister,
    pattern_table0: Box<[u8]>, // 0x1000 byte
    vram: Box<[u8]>,           // 2kb
}

impl PpuUnit {
    fn new() -> PpuUnit {
        let register = PpuRegister::default();
        let pattern_table0 = Box::new([0_u8; 0x1000]);
        let vram = Box::new([0u8; 0x2000]); // 2048 byte
        PpuUnit {
            register,
            pattern_table0,
            vram,
        }
    }

    fn load_from_cpu(&mut self, addr: u16, data: u8) {
        println!("ppu load_from_cpu: addr: {}, data: {}", addr, data);
        match addr {
            0 => {
                self.register.ctrl = data;
            }
            1 => {
                self.register.mask = data;
            }
            2 => {
                panic!("Try to write read only register on ppu");
            }
            3 => {
                self.register.oamaddr = data;
            }
            4 => {
                self.register.oamdata = data;
            }
            5 => {
                self.register.scroll = data;
            }
            6 => {
                let w = data as u16;
                if !self.register.toggle_ppuaddr {
                    self.register.ppuaddr = (self.register.ppuaddr & 0xff) | w << 8;
                } else {
                    self.register.ppuaddr = (self.register.ppuaddr & 0xff00) | w;
                }
                println!(
                    "ppuaddr write: {:x}, toggle_ppuaddr: {}, w: {:x}",
                    self.register.ppuaddr, self.register.toggle_ppuaddr, w
                );
                self.register.toggle_ppuaddr = !self.register.toggle_ppuaddr;
            }
            7 => {
                let addr = self.register.ppuaddr;
                self.write_to_ppu(addr, data);

                //println!("ppudata write: addr {:x}, data: {:x}", addr, data);

                // アドレスのインクリメント
                let inc = if self.register.ctrl & 0x4 == 0 { 1 } else { 32 };
                self.register.ppuaddr += inc;
            }
            _ => {
                assert!(false, "yet to be implemented");
            }
        }
    }

    fn write_to_ppu(&mut self, addr: u16, data: u8) {
        println!("write_to_ppu: {:x}, {:x}", addr, data);

        match addr {
            addr if addr < 0x1000 => {
                println!("ppu pattern_table0 write: {:x}, {:x}", addr, data);
                self.pattern_table0[addr as usize] = data;
            }
            addr if addr >= 0x2000 && addr < 0x2000 + 0x400 => {
                println!("ppu name_table0 write: {:x}, {:x}", addr, data);
                // TODO
            }
            addr if addr >= 0x3f00 && addr < 0x3f00 + 0x20 => {
                println!("ppu pallet_ram_indices write: {:x}, {:x}", addr, data);
                // TODO
            }
            _ => {
                panic!("yet to be implemented to write addr: {:x}", addr);
            }
        }
    }
}

struct Machine {
    register: Register,
    rom: Rom,
    wram: Box<[u8]>, // 2kb
    ppu_unit: PpuUnit,
}

//struct Bus { }

impl Machine {
    fn new(rom: Rom) -> Machine {
        let register = Register::default();
        let wram = Box::new([0; 2 * 1024]);
        let ppu_unit = PpuUnit::new();
        Machine {
            register,
            rom,
            wram,
            ppu_unit,
        }
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

        assert!(false, "yet to be implemented");
        return 0;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        let word = self.read_word(addr);
        (word & 0x00ff) as u8
    }

    fn write_byte(&mut self, addr: u16, data: u8) {
        // TODO
        if addr >= 0x2000 && addr < 0x2008 {
            let a = addr - 0x2000;
            self.ppu_unit.load_from_cpu(a, data);
            return;
        }
        assert!(false, "yet to be implemented");
    }

    fn fetch_byte(&mut self) -> u8 {
        let v = self.read_byte(self.register.pc);
        self.register.pc += 1;
        v
    }

    fn fetch_word(&mut self) -> u16 {
        let v = self.read_word(self.register.pc);
        self.register.pc += 2;
        v
    }

    fn reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.read_word(0xfffc);
    }

    fn execute(&mut self) {
        let pc = self.register.pc;
        let op = self.read_byte(pc);
        self.register.pc += 1;

        //println!("XXX op: {:x} from {:x}", op, self.register.pc);

        match op {
            0x4c => {
                // JMP Absolute
                // 間接アドレス指定がページをまたいでいる場合、アドレスの指定は失敗するらしい
                // http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#instruction
                let abs = self.fetch_word();
                println!("JMP: abs: {}, from: {}", abs, self.register.pc);
                self.register.pc = abs;
            }
            0x78 => {
                // SEI
                self.register.p.interrupt = true;
            }
            0x88 => {
                // DEY
                match self.register.y {
                    0 => {
                        self.register.y = 0xff;
                    }
                    1 => {
                        self.register.p.zero = true;
                        self.register.y -= 1;
                    }
                    _ => {
                        self.register.y -= 1;
                    }
                }
            }
            0x8d => {
                // STA Absolute
                let addr = self.fetch_word();
                let d = self.register.a;
                self.write_byte(addr, d);
            }
            0x9a => {
                // TXS
                self.register.s = self.register.x;
            }
            0xa0 => {
                // LDY
                let imm = self.fetch_byte();
                self.register.y = imm;
                //println!("LDY imm: {:x}", imm);
            }
            0xa2 => {
                // LDX
                let imm = self.fetch_byte();
                self.register.x = imm;
                //println!("LDX imm: {:x}", imm);
            }
            0xa9 => {
                // LDA
                let imm = self.fetch_byte();
                self.register.a = imm;
                //println!("LDA imm: {:x}", imm);
            }
            0xbd => {
                // LDA Absolute, X
                let abs = self.fetch_word();
                let x = self.register.x as u16;
                let addr = abs + x;
                let data = self.read_byte(addr);
                self.register.a = data;
                println!("LDA abs, x, addr: {}, {}, {}, data; {}", abs, x, addr, data);
            }
            0xe8 => {
                // INX
                if self.register.x == 0xff {
                    self.register.x = 0;
                    self.register.p.carry = true;
                } else {
                    self.register.x += 1;
                }
            }
            0xd0 => {
                // BNE
                let rel = u8_to_i8(self.fetch_byte());
                println!("BNE: {}", rel);
                if self.register.p.zero == false {
                    let pc = self.register.pc as i16 + rel as i16;
                    self.register.pc = pc as u16;
                }
            }

            _ => {
                // TODO
                println!("op yet to be implemented: {:x}", op);
            }
        }
    }

    fn run(&mut self) {
        self.reset();

        let mut inst_count: u32 = 0;
        let inst_count_max: u32 = 200;

        loop {
            self.execute();
            inst_count += 1;
            if inst_count >= inst_count_max {
                break;
            }
        }

        println!("Loop out with inst_count: {}", inst_count);
    }
}

#[test]
fn test_machine() {
    let rom = Rom::load_image("rom/sample1/sample1.nes".to_string());
    let mut machine = Machine::new(rom);

    println!("XXX: {:x?}", machine.read_word(0x8000));

    assert_eq!(0x00, machine.read_word(0));
    assert_eq!(0x00, machine.read_word(0));
    assert_eq!(0x00, machine.read_word(2));
    assert_eq!(0x8000, machine.read_word(0xfffc)); // reset
    assert_eq!(0xa278, machine.read_word(0x8000)); // prg
    assert_eq!(0x9aff, machine.read_word(0x8002)); // prg
    assert_eq!(0x00a9, machine.read_word(0x8004)); // prg
    assert_eq!(0x00, machine.read_byte(0xfffc)); // reset
    assert_eq!(0x80, machine.read_byte(0xfffd)); // reset

    machine.run()
}

fn dump_bin(path: &Path, bin: &[u8]) -> std::io::Result<()> {
    let mut fp = OpenOptions::new().create(true).write(true).open(path)?;
    fp.write_all(bin)?;
    Ok(())
}

fn main() {
    //println!("Hello, world!");

    let rom = Rom::load_image("rom/sample1/sample1.nes".to_string());

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
