// main.rs

#![allow(dead_code)]

use std::fs::File;
use std::fs::OpenOptions;
//use std::io::{BufReader, Read};
use std::io::prelude::*;
use std::io::{Cursor, Read};
use std::path::Path;

extern crate quicksilver;
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{
        //Background::{Col, Img},
        Background::Img,
        Color,
        Font,
        FontStyle,
        Image,
        PixelFormat,
    },
    input::Key,
    lifecycle::{run_with, Asset, Settings, State, Window},
    Future, Result,
};

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

extern crate image;

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
}

#[derive(Default)]
struct Rom {
    bin: Vec<u8>,
}

impl Rom {
    fn dummy() -> Self {
        Self { bin: Vec::new() }
    }

    fn load_image(filename: String) -> Box<Rom> {
        let fp = File::open(filename).unwrap();
        let mut bin: Vec<u8> = Vec::with_capacity(1024 * 1024);
        let mut reader = std::io::BufReader::new(fp);
        reader.read_to_end(&mut bin).unwrap();
        Box::new(Rom { bin: bin })
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
    pattern_table0: Box<[u8]>,  // 0x1000 byte
    name_table0: Box<[u8]>,     // 0x03c0 byte
    attr_table0: Box<[u8]>,     // 0x0040 byte
    bg_palette: [u8; 0x10],     // 0x0010 byte
    sprite_palette: [u8; 0x10], // 0x0010 byte
    vram: Box<[u8]>,            // 0x2000 byte
}

impl PpuUnit {
    fn new() -> PpuUnit {
        let register = PpuRegister::default();
        let pattern_table0 = Box::new([0_u8; 0x1000]);
        let name_table0 = Box::new([0_u8; 0x03c0]);
        let attr_table0 = Box::new([0_u8; 0x0040]);
        let bg_palette = [0_u8; 0x10];
        let sprite_palette = [0_u8; 0x10];
        let vram = Box::new([0u8; 0x2000]); // 2048 byte
        PpuUnit {
            register,
            pattern_table0,
            name_table0,
            attr_table0,
            bg_palette,
            sprite_palette,
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
            addr if addr >= 0x2000 && addr < 0x2000 + 0x3c0 => {
                let a = (addr - 0x2000) as usize;
                println!("ppu name_table0 write: {:x}, {:x}", addr, data);
                self.name_table0[a] = data;
            }
            addr if addr >= 0x23c0 && addr < 0x23c0 + 0x040 => {
                let a = (addr - 0x23c0) as usize;
                println!("ppu name_table0 write: {:x}, {:x}", addr, data);
                self.attr_table0[a] = data;
            }
            addr if addr >= 0x3f00 && addr < 0x3f00 + 0x10 => {
                let a = (addr - 0x3f00) as usize;
                self.bg_palette[a] = data;
                println!("ppu bg_palette write: {:x}, {:x}", addr, data);
            }
            addr if addr >= 0x3f10 && addr < 0x3f10 + 0x10 => {
                let a = (addr - 0x3f10) as usize;
                self.sprite_palette[a] = data;
                println!("ppu sprite_palette write: {:x}, {:x}", addr, data);
            }
            _ => {
                panic!("yet to be implemented to write addr: {:x}", addr);
            }
        }
    }
}

//#[derive(Default)]
struct Machine {
    register: Register,
    wram: Box<[u8]>, // 2kb
    ppu_unit: PpuUnit,
    rom: Box<Rom>,
    step_count: u32,
}

impl Machine {
    fn new() -> Machine {
        let register = Register::default();
        let wram = Box::new([0; 2 * 1024]);
        let ppu_unit = PpuUnit::new();
        let rom = Box::new(Rom::dummy());

        Machine {
            register,
            wram,
            ppu_unit,
            rom,
            step_count: 0,
        }
    }

    fn hard_reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.read_word(0xfffc);
    }

    fn set_rom(&mut self, rom: Box<Rom>) {
        self.rom = rom;
        self.hard_reset();
    }

    fn read_word(&self, addr: u16) -> u16 {
        if addr <= 0x07ff {
            let mut cur = Cursor::new(&self.wram);
            cur.set_position(addr as u64);
            return cur.read_u16::<LittleEndian>().unwrap();
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

    fn execute(&mut self) {
        self.step_count += 1;

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
                let c = self.register.y;
                self.register.y = if c == 0 { 0xff } else { c - 1 };
                self.register.p.zero = self.register.y == 0;
                self.register.p.negative = self.register.y & 0x80 != 0;
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

    fn step(&mut self) {
        self.execute();
    }

    // テスト用
    fn exec_loop(&mut self) {
        self.hard_reset();

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
    let mut machine = Machine::new();
    machine.set_rom(rom);

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

    machine.exec_loop()
}

fn dump_bin(path: &Path, bin: &[u8]) -> std::io::Result<()> {
    let mut fp = OpenOptions::new().create(true).write(true).open(path)?;
    fp.write_all(bin)?;
    Ok(())
}

#[derive(Default)]
struct PadState {
    key_ud: i8,
    key_lr: i8,
}

const DISPLAY_SIZE: (u32, u32) = (256, 240);
const FONT_NAME: &str = "mononoki-Regular.ttf";

#[derive(Default, Copy, Clone)]
struct Rgb(u8, u8, u8);

struct FrameBuffer {
    sizes: (u32, u32),
    buf: Vec<u8>,
    image: Image,
}

impl FrameBuffer {
    fn new(sizes: (u32, u32)) -> Self {
        let len = 3 * sizes.0 as usize * sizes.1 as usize;
        let buf = vec![0; len];
        let image = Image::from_raw(
            &buf,
            DISPLAY_SIZE.0 as u32,
            DISPLAY_SIZE.1 as u32,
            PixelFormat::RGB,
        )
        .unwrap();

        Self { sizes, buf, image }
    }

    fn clear(&mut self, rgb: Rgb) {
        let n = self.buf.len() / 3;
        for i in 0..n {
            self.buf[i * 3 + 0] = rgb.0;
            self.buf[i * 3 + 1] = rgb.1;
            self.buf[i * 3 + 2] = rgb.2;
        }
    }

    fn set_pixel(&mut self, pos: (u32, u32), rgb: (u8, u8, u8)) {
        let ofs = 3 * (pos.1 * self.sizes.0 + pos.0) as usize;
        self.buf[ofs + 0] = rgb.0;
        self.buf[ofs + 1] = rgb.1;
        self.buf[ofs + 2] = rgb.2;
    }

    fn apply_to_image(&mut self) {
        self.image = Image::from_raw(
            &self.buf,
            self.sizes.0 as u32,
            self.sizes.1 as u32,
            PixelFormat::RGB,
        )
        .unwrap();
    }

    fn draw(&mut self, window: &mut Window, rate: u32) {
        self.apply_to_image(); //
        let sz = (rate * self.sizes.0, rate * self.sizes.1);
        window.draw(&Rectangle::new((0, 0), sz), Img(&self.image));
    }

    fn draw_tile(
        &mut self,
        machine: &Machine,
        pos: (u32, u32),
        v: u8,
        palette: &[u8],
    ) {
        assert!(palette.len() == 4);

        let offset = v as u32 * 16;

        for y in 0..8_u32 {
            let idx0 = offset + y;
            let idx1 = offset + y + 8;

            let l0 = machine.rom.get_chr()[idx0 as usize];
            let l1 = machine.rom.get_chr()[idx1 as usize];

            for x in 0..8_u32 {
                let mask = 0x01_u8 << (7 - x);
                let b0 = if l0 & mask != 0 { 1 } else { 0 };
                let b1 = if l1 & mask != 0 { 1 } else { 0 };
                let color_idx = (b1 << 1) + b0;

                let rgb = COLOR_PALETTE[palette[color_idx] as usize];
                self.set_pixel((pos.0 + x, pos.1 + y), rgb);
            }
        }
    }

    // buf に書き込む
    fn render_line(&mut self, machine: &Machine, y: u32) {

        let name_table = &machine.ppu_unit.name_table0;
        let attr_table = &machine.ppu_unit.attr_table0;
        let bg_palette = &machine.ppu_unit.bg_palette;
        //let chr = &machine.rom.get_chr();

        let block_base = y % 32;
        let subblock_y = if y / 8 % 2 == 0 { 0 } else { 1 };

        let tile_base_idx = y / 8;


        // ブロックは一列に16個
        for ib in 0..16 {

            let block_idx = ib + block_base;
            let attr = attr_table[block_idx as usize];

            for subblock_x in 0..2 {
                let subblock_idx = (subblock_y << 1) | subblock_x;
                assert!(subblock_idx < 4, "subblock_idx is {}", subblock_idx);

                let palette_idx = attr >> (subblock_idx * 2) & 0x03;
                assert!(palette_idx < 4);

                let palette_ofs = (palette_idx * 4) as usize;
                let palette = &bg_palette[palette_ofs..palette_ofs + 4];

                for x_in_block in 0..8 {
                    let x = (8 * subblock_x) + (16 * ib) + x_in_block;
                    assert!(x < DISPLAY_SIZE.0);
                    assert!(y < DISPLAY_SIZE.1);
                    let pos = (x, y);

                    let tile_idx = (tile_base_idx + (x / 8)) as usize;

                    let name0 = name_table[16 * tile_idx];
                    let name1 = name_table[16 * tile_idx + 8];

                    let mask = 0x01u8 << (7 - x_in_block);
                    let b0 = if name0 & mask != 0 { 1 } else { 0 };
                    let b1 = if name1 & mask != 0 { 1 } else { 0 };
                    let color_idx = (b1 << 1) + b0;
                    let rgb = COLOR_PALETTE[palette[color_idx] as usize];
                    //frame_buffer.set_pixel((pos.0 + x, pos.1 + y), rgb);

                    //let rgb = [
                        //(0xff, 0, 0), (0x00, 0xff, 0),
                        //(0x00, 0, 0xff), (0xff, 0xff, 0),
                    //] [subblock_idx as usize]; 

                    self.set_pixel(pos, rgb);
                }

            }
        }
    }
}

struct App {
    pixel_rate: u32,
    display_size: (u32, u32),
    pad_state: PadState,
    machine: Machine,
    frame_buffer: FrameBuffer,
    font_line: Option<Asset<Image>>, // for debug
}

impl App {
    fn new() -> Result<Self> {
        let machine = Machine::new();

        // 疑似初期化
        Ok(Self {
            pixel_rate: 0,
            display_size: DISPLAY_SIZE,
            pad_state: PadState::default(),
            machine,
            frame_buffer: FrameBuffer::new((1, 1)),
            font_line: None,
        })
    }

    fn new_with_params(display_size: (u32, u32), pixel_rate: u32, rom: Box<Rom>) -> Result<Self> {
        let mut machine = Machine::new();
        machine.set_rom(rom);

        let font_line = Asset::new(
            Font::load(FONT_NAME)
                .and_then(|font| font.render("Jagajin", &FontStyle::new(20.0, Color::RED))),
        );

        let app = Self {
            pixel_rate,
            display_size,
            pad_state: PadState::default(),
            machine,
            frame_buffer: FrameBuffer::new(DISPLAY_SIZE),
            font_line: Some(font_line),
        };
        Ok(app)
    }

    fn draw_tile(
        frame_buffer: &mut FrameBuffer,
        machine: &Machine,
        pos: (u32, u32),
        v: u8,
        palette: &[u8],
    ) {
        assert!(palette.len() == 4);

        let offset = v as u32 * 16;

        for y in 0..8_u32 {
            let idx0 = offset + y;
            let idx1 = offset + y + 8;

            let l0 = machine.rom.get_chr()[idx0 as usize];
            let l1 = machine.rom.get_chr()[idx1 as usize];

            for x in 0..8_u32 {
                let mask = 0x01_u8 << (7 - x);
                let b0 = if l0 & mask != 0 { 1 } else { 0 };
                let b1 = if l1 & mask != 0 { 1 } else { 0 };
                let color_idx = (b1 << 1) + b0;

                let rgb = COLOR_PALETTE[palette[color_idx] as usize];
                frame_buffer.set_pixel((pos.0 + x, pos.1 + y), rgb);
            }
        }
    }

    fn draw_internal(frame_buffer: &mut FrameBuffer, machine: &Machine) {
        let name_table = &machine.ppu_unit.name_table0;
        let attr_table = &machine.ppu_unit.attr_table0;
        let bg_palette = &machine.ppu_unit.bg_palette;

        for (i, v) in name_table.iter().enumerate() {
            // タイルは一列32個
            let tile_x = (i % 0x20) as u32;
            let tile_y = (i / 0x20) as u32;

            let pixel_pos = (tile_x * 8, tile_y * 8);

            // block
            let block_pos = (pixel_pos.0 / 16, pixel_pos.1 / 16);

            let block_idx = (block_pos.1 / 2) + (block_pos.0 / 2);

            // [0, 4): Zの字
            let subblock_idx = (tile_x % 2) + (tile_y % 2 * 2);
            assert!(subblock_idx < 4);

            let attr = attr_table[block_idx as usize];
            let palette_idx = attr >> (subblock_idx * 2) & 0x03;
            assert!(palette_idx < 4);

            let palette_ofs = (palette_idx * 4) as usize;
            let palette = &bg_palette[palette_ofs..palette_ofs + 4];

            App::draw_tile(frame_buffer, machine, pixel_pos, *v, palette);
        }
    }

    fn run(rom: Box<Rom>) {
        let pixel_rate = 2;
        let display_size = (DISPLAY_SIZE.0 * pixel_rate, DISPLAY_SIZE.1 * pixel_rate);
        let v = Vector::new(display_size.0, display_size.1);
        run_with("RUSTNESS", v, Settings::default(), || {
            Self::new_with_params(display_size, pixel_rate, rom)
        });
    }
}

impl State for App {
    fn new() -> Result<App> {
        App::new()
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Q].is_down() {
            window.close();
        }

        self.pad_state = PadState::default();
        if window.keyboard()[Key::K].is_down() {
            self.pad_state.key_ud = -1;
        }
        if window.keyboard()[Key::J].is_down() {
            self.pad_state.key_ud = 1;
        }
        if window.keyboard()[Key::H].is_down() {
            self.pad_state.key_lr = -1;
        }
        if window.keyboard()[Key::L].is_down() {
            self.pad_state.key_lr = 1;
        }

        self.machine.step();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.frame_buffer.clear(Rgb(0, 0xcc, 0));

        if true {
            for y in 0..DISPLAY_SIZE.1 {
                self.frame_buffer.render_line(&self.machine, y);
            }
        } else {
            App::draw_internal(&mut self.frame_buffer, &self.machine);
            //self.frame_buffer.set_pixel((10, 10), (0xff, 0, 0)); // debug
        }
        self.frame_buffer.draw(window, self.pixel_rate);

        if false {
            // デバッグ用のフォント文字列があれば描画する
            if let Some(ref mut font_line) = self.font_line {
                font_line.execute(|image| {
                    window.draw(&image.area().with_center((100, 100)), Img(&image));
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}

fn main() {
    let rom = Rom::load_image("rom/sample1/sample1.nes".to_string());
    App::run(rom);
}

const COLOR_PALETTE: [(u8, u8, u8); 64] = [
    (84, 84, 84),
    (0, 30, 116),
    (8, 16, 144),
    (48, 0, 136),
    (68, 0, 100),
    (92, 0, 48),
    (84, 4, 0),
    (60, 24, 0),
    (32, 42, 0),
    (8, 58, 0),
    (0, 64, 0),
    (0, 60, 0),
    (0, 50, 60),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (152, 150, 152),
    (8, 76, 196),
    (48, 50, 236),
    (92, 30, 228),
    (136, 20, 176),
    (160, 20, 100),
    (152, 34, 32),
    (120, 60, 0),
    (84, 90, 0),
    (40, 114, 0),
    (8, 124, 0),
    (0, 118, 40),
    (0, 102, 120),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (76, 154, 236),
    (120, 124, 236),
    (176, 98, 236),
    (228, 84, 236),
    (236, 88, 180),
    (236, 106, 100),
    (212, 136, 32),
    (160, 170, 0),
    (116, 196, 0),
    (76, 208, 32),
    (56, 204, 108),
    (56, 180, 204),
    (60, 60, 60),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (168, 204, 236),
    (188, 188, 236),
    (212, 178, 236),
    (236, 174, 236),
    (236, 174, 212),
    (236, 180, 176),
    (228, 196, 144),
    (204, 210, 120),
    (180, 222, 120),
    (168, 226, 144),
    (152, 226, 180),
    (160, 214, 228),
    (160, 162, 160),
    (0, 0, 0),
    (0, 0, 0),
];
