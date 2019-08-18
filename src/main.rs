// main.rs

#![allow(dead_code)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

extern crate rustness;
use rustness::rom::Rom;

//use rustness::ppu::RGB; // TODO: remove this
//use rustness::ppu::color_palette::COLOR_PALETTE; // TODO: remove this

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
        //println!("ppu load_from_cpu: addr: {}, data: {}", addr, data);
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
                let w = u16::from(data);
                if !self.register.toggle_ppuaddr {
                    self.register.ppuaddr = (self.register.ppuaddr & 0xff) | w << 8;
                } else {
                    self.register.ppuaddr = (self.register.ppuaddr & 0xff00) | w;
                }
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
                panic!("yet to be implemented");
            }
        }
    }

    fn write_to_ppu(&mut self, addr: u16, data: u8) {
        //println!("write_to_ppu: {:x}, {:x}", addr, data);

        //fn chk(addr: u16, base: u16, size: u16) -> bool {
        //return addr >= base && addr < base + size;
        //}

        // solve mirror mapping
        let addr2 = if addr >= 0x3000 && addr <= 0x3eff {
            addr - 0x1000
        } else if addr >= 0x3f20 && addr <= 0x3fff {
            addr - 0x0020
        } else {
            addr
        };

        match addr2 {
            a if a < 0x1000 => {
                //println!("ppu pattern_table0 write: {:x}, {:x}", a, data);
                self.pattern_table0[a as usize] = data;
            }
            a if a >= 0x2000 && a < 0x2000 + 0x3c0 => {
                let idx = (a - 0x2000) as usize;
                //println!("ppu name_table0 write: {:x}, {:x}", a, data);
                self.name_table0[idx] = data;
            }
            a if a >= 0x23c0 && a < 0x23c0 + 0x040 => {
                let idx = (a - 0x23c0) as usize;
                //println!("ppu name_table0 write: {:x}, {:x}", a, data);
                self.attr_table0[idx] = data;
            }
            a if a >= 0x3f00 && a < 0x3f00 + 0x10 => {
                let idx = (a - 0x3f00) as usize;
                self.bg_palette[idx] = data;
                //println!("ppu bg_palette write: {:x}, {:x}, {:x}", a, addr2, data);
            }
            a if a >= 0x3f10 && a < 0x3f10 + 0x10 => {
                let idx = (a - 0x3f10) as usize;
                self.sprite_palette[idx] = data;
                //println!("ppu sprite_palette write: {:x}, {:x}", a, data);
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

    fn set_rom(&mut self, rom: Rom) {
        self.rom = Box::new(rom);
        self.hard_reset();
    }

    fn read_word(&self, addr: u16) -> u16 {
        if addr <= 0x07ff {
            let mut cur = Cursor::new(&self.wram);
            cur.set_position(u64::from(addr));
            return cur.read_u16::<LittleEndian>().unwrap();
        }

        // 0xffff まで有効
        if addr >= 0x8000 {
            let base = 0x8000;
            let prg_size = self.rom.get_prg().len();
            let mut ofs = u64::from(addr - base);

            // prg が 16kb しかない場合は、0xc000 からの領域にミラーリングされていると見なす
            if ofs >= 0x4000 && prg_size <= 0x4000 {
                ofs -= 0x4000;
            }

            let mut cur = Cursor::new(self.rom.get_prg());
            cur.set_position(ofs);
            return cur.read_u16::<LittleEndian>().unwrap();
        }

        assert!(false, "yet to be implemented: {:x}", addr);
        0
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

    fn execute(&mut self) -> u32 {
        let cycle = 3;
        let pc = self.register.pc;
        let op = self.read_byte(pc);
        self.register.pc += 1;

        match op {
            0x4c => {
                // JMP Absolute
                // 間接アドレス指定がページをまたいでいる場合、アドレスの指定は失敗するらしい
                // http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#instruction
                let abs = self.fetch_word();
                //println!("JMP: abs: {}, from: {}", abs, self.register.pc);
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
            0xad => {
                // LDA Abs
                let addr = self.fetch_word();
                let data = self.read_byte(addr);
                self.register.a = data;
                //println!("LDA abs: {:x}, {:x}", addr, data);
            }
            0xbd => {
                // LDA Absolute, X
                let abs = self.fetch_word();
                let x = u16::from(self.register.x);
                let addr = abs + x;
                let data = self.read_byte(addr);
                self.register.a = data;
                //println!("LDA abs, x, addr: {}, {}, {}, data; {}", abs, x, addr, data);
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
                if !self.register.p.zero {
                    let pc = self.register.pc as i16 + i16::from(rel);
                    self.register.pc = pc as u16;
                }
            }

            _ => {
                // TODO
                panic!("op yet to be implemented: {:x}", op);
            }
        }

        cycle
    }

    fn step(&mut self, frame_buffer: &mut FrameBuffer) {
        self.step_count += 1;
        //println!("step: {}", self.step_count);

        for y in 0..DISPLAY_SIZE.1 {
            let mut cycle = 0;

            loop {
                cycle += self.execute();
                if cycle > 341 {
                    break;
                }
            }

            frame_buffer.render_line(&self, y);
        }
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
    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    let mut machine = Machine::new();
    machine.set_rom(rom);

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
            self.buf[i * 3] = rgb.0;
            self.buf[i * 3 + 1] = rgb.1;
            self.buf[i * 3 + 2] = rgb.2;
        }
    }

    fn set_pixel(&mut self, pos: (u32, u32), rgb: (u8, u8, u8)) {
        let ofs = 3 * (pos.1 * self.sizes.0 + pos.0) as usize;
        self.buf[ofs] = rgb.0;
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

    // buf に書き込む
    fn render_line(&mut self, machine: &Machine, y: u32) {
        let name_table = &machine.ppu_unit.name_table0;
        let attr_table = &machine.ppu_unit.attr_table0;
        let bg_palette = &machine.ppu_unit.bg_palette;
        let chr_table = &machine.rom.get_chr();

        let block_base = y % 32;
        let subblock_y = if y / 8 % 2 == 0 { 0 } else { 1 };

        let tile_base_idx = (y / 8) * 32;
        let y_in_tile = (y % 8) as usize;

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

                let tile_idx = (tile_base_idx + 2 * ib + subblock_x) as usize;
                let chr_idx = name_table[tile_idx] as usize;
                let chr_ofs = 16 * chr_idx + y_in_tile;
                let name0 = chr_table[chr_ofs];
                let name1 = chr_table[chr_ofs + 8];

                let x_base = (8 * subblock_x) + (16 * ib);

                for x_in_block in 0..8 {
                    let x = x_base + x_in_block;
                    assert!(x < DISPLAY_SIZE.0);
                    assert!(y < DISPLAY_SIZE.1);
                    let pos = (x, y);

                    let mask = 0x01u8 << (7 - x_in_block);
                    let b0 = if name0 & mask != 0 { 1 } else { 0 };
                    let b1 = if name1 & mask != 0 { 1 } else { 0 };
                    let color_idx = (b1 << 1) + b0;

                    //let rgb = COLOR_PALETTE[palette[color_idx] as usize];
                    //self.set_pixel(pos, rgb);
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

    fn new_with_params(display_size: (u32, u32), pixel_rate: u32, rom: Rom) -> Result<Self> {
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

    fn run(rom: Rom) {
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

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        //window.clear(Color::BLACK)?;

        // 本来 update() で呼び出すべきかもしれないが、ここに書かないとキーイベントなどがうまくとれない.
        self.machine.step(&mut self.frame_buffer);

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
    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    //let rom = Rom::load_image("static/roms/giko005.nes".to_string());
    App::run(rom);
}
