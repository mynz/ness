#![allow(dead_code)]

mod frame_buffer;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240 + 20;

// 240 ライン目からVBLANK
const VBLANK_AHEAD: u32 = 241;

// １ラインに掛かるサイクル数
const CYCLES_PER_LINE: u32 = 341;

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

    cur_exec_cycles: u32, // x
    cur_line_exec: u32,   // y
    rest_cycles_line: u32,
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

            cur_exec_cycles: 0,
            cur_line_exec: 0,
            rest_cycles_line: CYCLES_PER_LINE,
        }
    }

    pub fn get_cur_exec_pos(&self) -> (u32, u32) {
        (self.cur_exec_cycles, self.cur_line_exec)
    }

    pub fn execute(&mut self, cycles: u32) {
        // 1 frame = 341 * 262 = 89342 PPU cycles
        // http://taotao54321.hatenablog.com/entry/2017/04/11/115205

        // TODO
        assert!(cycles <= CYCLES_PER_LINE);

        self.cur_exec_cycles = (self.cur_exec_cycles + cycles) % CYCLES_PER_LINE;

        if self.rest_cycles_line > cycles {
            // まだラインの処理中.
            self.rest_cycles_line -= cycles;
        } else {
            // ライン処理の終了を検知.
            let delta = cycles - self.rest_cycles_line;
            self.rest_cycles_line = CYCLES_PER_LINE - delta;

            // ラインを進める
            self.cur_line_exec += 1;
            if self.cur_line_exec == 262 {
                // ライン0へ折り返す.
                self.cur_line_exec = 0;
            }
        }
        // 241ラインからVBLANK
        self.reg.status.vblank = self.cur_line_exec == 241;
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
                let w = data as u16;
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
    fn test_vblank0() {
        let mut ppu = PpuUnit::new();
        let mut count = 0;
        loop {
            count += 1;
            ppu.execute(CYCLES_PER_LINE);
            if ppu.reg.status.vblank {
                break;
            }
        }
        assert!(true);
        assert_eq!(count, 241);
    }

    #[test]
    fn test_vblank1() {
        let mut ppu = PpuUnit::new();
        let mut cycles = 0;
        let mut count = 0;
        for _ in 0..262 {
            ppu.execute(CYCLES_PER_LINE);
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
