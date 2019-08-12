#![allow(dead_code)]

mod frame_buffer;

use self::frame_buffer::FrameBuffer;
use crate::rom::Rom;
use crate::Pos;
use std::path::Path;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240 + 20;

const DISPLAY_SIZE: (u32, u32) = (256, 240);

// 240 ライン目からVBLANK
//const VBLANK_AHEAD: u32 = 241;

// １ラインに掛かるサイクル数
const CYCLES_PER_LINE: u32 = 341;

pub struct RGB(u8, u8, u8);

#[derive(Default)]
struct Status {
    vblank: bool,
    sprite_hit: bool,
}

#[derive(Default)]
pub struct PpuRegister {
    pub ctrl: u8,     // w
    pub mask: u8,     // w
    status: Status,   // r
    pub oamaddr: u8,  // w
    pub oamdata: u8,  // rw
    pub scroll: u8,   // w
    pub ppuaddr: u16, // w
    pub ppudata: u8,  // rw

    toggle_ppuscroll: bool, // for scroll
    toggle_ppuaddr: bool,   // for addr
    oamdata_counter: u8,    // [0-3] for oamdata
}

impl PpuRegister {}

pub struct PpuUnit {
    reg: PpuRegister,
    pattern_table0: Box<[u8]>, // 0x1000 byte (4kb), from 0x0000, スプライト用パターンテーブル
    name_table0: Box<[u8]>,    // 0x03c0 byte
    attr_table0: Box<[u8]>,    // 0x0040 byte
    bg_palette: [u8; 0x10],    // 0x0010 byte
    sprite_palette: [u8; 0x10], // 0x0010 byte
    vram: Box<[u8]>,           // 0x2000 byte

    frame_buffer: FrameBuffer,

    cur_render_x: u32, // x
    cur_render_y: u32,   // y
    rest_cycles_in_line: u32,
}

impl PpuUnit {
    pub fn new() -> PpuUnit {
        let reg = PpuRegister::default();
        let pattern_table0 = Box::new([0_u8; 0x1000]);
        let name_table0 = Box::new([0_u8; 0x03c0]);
        let attr_table0 = Box::new([0_u8; 0x0040]);
        let bg_palette = [0_u8; 0x10];
        let sprite_palette = [0_u8; 0x10];
        let vram = Box::new([0u8; 0x2000]); // 2048 byte
        PpuUnit {
            reg,
            pattern_table0,
            name_table0,
            attr_table0,
            bg_palette,
            sprite_palette,
            vram,
            frame_buffer: FrameBuffer::new(DISPLAY_SIZE.0, DISPLAY_SIZE.1),
            cur_render_x: 0,
            cur_render_y: 0,
            rest_cycles_in_line: 0,
        }
    }

    pub fn get_cur_exec_pos(&self) -> (u32, u32) {
        (self.cur_render_x, self.cur_render_y)
    }

    fn render(&mut self, _pos: &Pos, _pixel_count: u32, _rom: &Rom) { }

    pub fn execute(&mut self, cycles: u32, rom: &Rom) {
        // 1 frame = 341 * 262 = 89342 PPU cycles
        // http://taotao54321.hatenablog.com/entry/2017/04/11/115205

        assert!(cycles <= CYCLES_PER_LINE);

        if cycles <= self.rest_cycles_in_line {
            // まだラインの処理中.

            let pos_ofs = Pos(self.cur_render_x, self.cur_render_y);
            self.render(&pos_ofs, cycles, rom);

            self.cur_render_x += cycles;
            self.rest_cycles_in_line -= cycles;
        } else {
            // ライン処理の終了を検知.
            let excess = cycles - self.rest_cycles_in_line;

            // ラインの残りをレンダリングする.
            if self.rest_cycles_in_line > 0 {
                let pos_ofs = Pos(self.cur_render_x, self.cur_render_y);
                self.render(&pos_ofs, self.rest_cycles_in_line, rom);
            }

            // 次のラインに進める.
            self.cur_render_y += 1;
            if self.cur_render_y >= 262 {
                // ライン0へ折り返す.
                self.cur_render_y = 0;
            }

            // 新ラインの初期化処理を行う.
            {
                // TODO

                // 241ラインからVBLANK
                self.reg.status.vblank = self.cur_render_y == 241;
            }

            // 新しいラインのレンダリング.
            let pos_ofs = Pos(0, self.cur_render_y);
            self.render(&pos_ofs, excess, rom);

            self.cur_render_x = excess;
            self.rest_cycles_in_line = CYCLES_PER_LINE - excess;
        }
    }

    pub fn get_ppu_register(&self) -> &PpuRegister {
        &self.reg
    }

    pub fn store_from_cpu(&mut self, addr: u16, data: u8) {
        //println!("xxx store_from_cpu: addr: {}, data: {}", addr, data);

        match addr {
            0x2000 => {
                self.reg.ctrl = data;
            }
            0x2001 => {
                self.reg.mask = data;
            }
            0x2002 => {
                panic!("Try to write read only register on ppu");
            }
            0x2003 => {
                self.reg.oamaddr = data;
            }
            0x2004 => {
                self.reg.oamdata = data;
            }
            0x2005 => {
                self.reg.scroll = data;
            }
            0x2006 => {
                let w = u16::from(data);
                if !self.reg.toggle_ppuaddr {
                    self.reg.ppuaddr = (self.reg.ppuaddr & 0xff) | w << 8;
                } else {
                    self.reg.ppuaddr = (self.reg.ppuaddr & 0xff00) | w;
                }
                self.reg.toggle_ppuaddr = !self.reg.toggle_ppuaddr;
            }
            0x2007 => {
                let ppuaddr = self.reg.ppuaddr;
                self.store_memory(ppuaddr, data);

                // アドレスのインクリメント
                let inc = if self.reg.ctrl & 0x4 == 0 { 1 } else { 32 };
                self.reg.ppuaddr += inc;
            }
            _ => {
                assert!(false, "yet to be implemented");
            }
        }
    }

    pub fn load_byte(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => {
                let mut r = 0;
                if self.reg.status.vblank {
                    r |= 0x80
                }
                if self.reg.status.sprite_hit {
                    r |= 0x40
                }

                self.reg.status.vblank = false;
                self.reg.toggle_ppuaddr = false;
                self.reg.toggle_ppuscroll = false;
                return r;
            }
            _ => {
                panic!("yet to implement ppu addr: {}", addr);
            }
        }

        //panic!("yet to be implemented: {:x}", addr);
    }

    pub fn load_word(&mut self, addr: u16) -> u16 {
        panic!("yet to be implemented: {:x}", addr);
    }

    fn store_memory(&mut self, addr: u16, data: u8) {
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
                self.pattern_table0[a as usize] = data;
            }
            a if a >= 0x2000 && a < 0x2000 + 0x3c0 => {
                let idx = (a - 0x2000) as usize;
                self.name_table0[idx] = data;
            }
            a if a >= 0x23c0 && a < 0x23c0 + 0x040 => {
                let idx = (a - 0x23c0) as usize;
                self.attr_table0[idx] = data;
            }
            a if a >= 0x3f00 && a < 0x3f00 + 0x10 => {
                let idx = (a - 0x3f00) as usize;
                self.bg_palette[idx] = data;
            }
            a if a >= 0x3f10 && a < 0x3f10 + 0x10 => {
                let idx = (a - 0x3f10) as usize;
                self.sprite_palette[idx] = data;
            }
            _ => {
                panic!("yet to be implemented to write addr: {:x}", addr);
            }
        }
    }

    #[allow(unused)]
    fn render_line(&self, frame_buffer: &mut FrameBuffer, rom: &Rom, pos: (u32, u32)) {
        // TODO
        //use crate::color_palette::COLOR_PALETTE;

        //let chr_table = &rom.get_chr();

        let c = (0xff, 0x00, 0x00);
        frame_buffer.set_pixel(pos, &RGB(c.0, c.1, c.2));
    }

    /*
    fn render_line(&self, frame_buffer: &mut FrameBuffer, chr_table: &[u8], y: u32) {
        let name_table = &self.name_table0;
        let attr_table = &self.attr_table0;
        let bg_palette = &self.bg_palette;
        //let chr_table = &machine.rom.get_chr();

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
                    let c = COLOR_PALETTE[palette[color_idx] as usize];

                    frame_buffer.set_pixel(pos, &RGB(c.0, c.1, c.2));
                }
            }
        }
    }
    */

    pub fn save_as_png<P: AsRef<Path>>(&self, path: P) {
        self.frame_buffer.save_as_png(path);
    }
}

impl Default for PpuUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ppu() {
        assert!(true);
    }

    #[test]
    fn test_pos() {
        let rom = Rom::dummy();

        {
            let mut ppu = PpuUnit::new();
            assert_eq!(ppu.get_cur_exec_pos(), (0, 0));
            ppu.execute(0, &rom);
            assert_eq!(ppu.get_cur_exec_pos(), (0, 0));
        }

        {
            let mut ppu = PpuUnit::new();
            ppu.execute(1, &rom);
            assert_eq!(ppu.get_cur_exec_pos(), (1, 0));
        }

    }

    #[test]
    fn test_vblank0() {
        let rom = Rom::dummy();
        let mut ppu = PpuUnit::new();
        let mut count = 0;
        loop {
            count += 1;
            ppu.execute(CYCLES_PER_LINE, &rom);
            if ppu.reg.status.vblank {
                break;
            }
        }
        assert!(true);
        assert_eq!(count, 241);
    }

    #[test]
    #[ignore]
    fn test_vblank1() {
        let rom = Rom::dummy();
        let mut ppu = PpuUnit::new();
        let mut cycles = 0;
        let mut count = 0;
        for _ in 0..262 {
            ppu.execute(CYCLES_PER_LINE, &rom);
            count += 1;
            cycles += CYCLES_PER_LINE;
        }
        assert!(true);
        assert_eq!(count, 262);
        assert_eq!(cycles, 89342);

        let pos = ppu.get_cur_exec_pos();
        assert_eq!(pos, (0, 0));
    }
}
