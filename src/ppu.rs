#![allow(dead_code)]

use crate::color_palette::COLOR_PALETTE;
use crate::frame_buffer::FrameBuffer;
use crate::rom::Rom;
use crate::{Cycle, Pos};
use std::path::Path;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

const DISPLAY_SIZE: (u32, u32) = (256, 240);

// 240 ライン目からVBLANK
//const VBLANK_AHEAD: u32 = 241;

// １ラインに掛かるサイクル数
const CYCLES_PER_LINE: u32 = 341;

const NAME_SIZE: u16 = 0x03c0;
const ATTR_SIZE: u16 = 0x0040;

const NM0_BASE: u16 = 0x2000;
const AT0_BASE: u16 = 0x23c0;
const NM1_BASE: u16 = 0x2400;
const AT1_BASE: u16 = 0x27c0;
const NM2_BASE: u16 = 0x2800;
const AT2_BASE: u16 = 0x2bc0;
const NM3_BASE: u16 = 0x2c00;
const AT3_BASE: u16 = 0x2fc0;

#[derive(Default)]
struct Status {
    vblank: bool,
    sprite_hit: bool,
}

#[derive(Default)]
pub struct PpuRegister {
    pub ctrl: u8,         // w
    pub mask: u8,         // w
    status: Status,       // r
    pub oamaddr: u8,      // w
    pub scroll: (u8, u8), // w
    pub ppuaddr: u16,     // w
    pub ppudata: u8,      // rw

    toggle_2005_2006: bool, // for addr
}

impl PpuRegister {}

// パターン
fn access_pat(pat: &[u8], pos_in_pat: Pos, h_mir: bool) -> u8 {
    let Pos(x, y) = pos_in_pat;
    let l0 = pat[y as usize];
    let l1 = pat[y as usize + 8];

    let sft = if h_mir { x } else { 7 - x };
    let p0 = (l0 >> sft) & 0x1;
    let p1 = (l1 >> sft) & 0x1;

    (p1 << 0x1) + p0
}

fn access_attr(attr: u8, pos_in_screen: &Pos) -> u8 {
    let Pos(x, y) = *pos_in_screen;
    let s0 = (x / 16) % 2;
    let s1 = (y / 16) % 2;
    let idx = s1 * 2 + s0; // [0, 3]
    let shift = 2 * (3 - idx); // 2 bit each
    let a = (attr >> shift) & 0x3;
    a
}

#[derive(Copy, Clone, Default)]
struct Sprite {
    y: u8,
    tile: u8,
    attr: u8,
    x: u8,
}

pub struct PpuUnit {
    reg: PpuRegister,

    name_table0: Box<[u8]>, // 0x03c0 byte
    attr_table0: Box<[u8]>, // 0x0040 byte
    name_table1: Box<[u8]>, // 0x03c0 byte
    attr_table1: Box<[u8]>, // 0x0040 byte
    name_table2: Box<[u8]>, // 0x03c0 byte
    attr_table2: Box<[u8]>, // 0x0040 byte
    name_table3: Box<[u8]>, // 0x03c0 byte
    attr_table3: Box<[u8]>, // 0x0040 byte

    bg_palette: [u8; 0x10],     // 0x0010 byte
    sprite_palette: [u8; 0x10], // 0x0010 byte
    vram: Box<[u8]>,            // 0x2000 byte

    sprites: Box<[Sprite]>, // 64 elems: 256 bytes

    line_sprites: [Sprite; 8],
    line_sprite_count: u8,

    nmi_interuption: bool,

    frame_buffer: FrameBuffer,

    next_render_x: u32, // x
    next_render_y: u32, // y
    rest_cycles_in_line: u32,

    frame_count: u32,
}

impl PpuUnit {
    pub fn new() -> PpuUnit {
        let reg = PpuRegister::default();
        let bg_palette = [0_u8; 0x10];
        let sprite_palette = [0_u8; 0x10];
        let vram = Box::new([0u8; 0x2000]); // 2048 byte
        let sprites = Box::new([Sprite::default(); 64]);

        const NAME_TABLE_SIZE: usize = 0x03c0;
        const ATTR_TABLE_SIZE: usize = 0x0040;

        PpuUnit {
            reg,
            name_table0: Box::new([0; NAME_TABLE_SIZE]),
            attr_table0: Box::new([0; ATTR_TABLE_SIZE]),
            name_table1: Box::new([0; NAME_TABLE_SIZE]),
            attr_table1: Box::new([0; ATTR_TABLE_SIZE]),
            name_table2: Box::new([0; NAME_TABLE_SIZE]),
            attr_table2: Box::new([0; ATTR_TABLE_SIZE]),
            name_table3: Box::new([0; NAME_TABLE_SIZE]),
            attr_table3: Box::new([0; ATTR_TABLE_SIZE]),
            bg_palette,
            sprite_palette,
            vram,
            sprites,
            line_sprites: [Sprite::default(); 8],
            line_sprite_count: 0,
            nmi_interuption: false,
            frame_buffer: FrameBuffer::new(DISPLAY_SIZE.0, DISPLAY_SIZE.1),
            next_render_x: 0,
            next_render_y: 0,
            rest_cycles_in_line: CYCLES_PER_LINE,
            frame_count: 0,
        }
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn get_next_render_pos(&self) -> Pos {
        Pos(self.next_render_x, self.next_render_y)
    }

    pub fn execute(&mut self, cycles: Cycle, rom: &Rom) {
        let mut rest_cycles = cycles;
        while rest_cycles > 0 {
            if rest_cycles > CYCLES_PER_LINE {
                self.execute_internal(CYCLES_PER_LINE, rom);
                rest_cycles -= CYCLES_PER_LINE;
            } else {
                self.execute_internal(rest_cycles, rom);
                break;
            }
        }
    }

    fn execute_internal(&mut self, cycles: Cycle, rom: &Rom) {
        // 1 frame = 341 * 262 = 89342 PPU cycles
        // http://taotao54321.hatenablog.com/entry/2017/04/11/115205

        assert!(cycles > 0);
        assert!(cycles <= CYCLES_PER_LINE);

        if cycles < self.rest_cycles_in_line {
            // まだラインの処理中.

            let pos_ofs = Pos(self.next_render_x, self.next_render_y);
            self.render(&pos_ofs, cycles, rom);

            self.next_render_x += cycles;
            self.rest_cycles_in_line -= cycles;
        } else {
            // ライン処理の終了を検知.
            let excess = cycles - self.rest_cycles_in_line;

            // ラインの残りをレンダリングする.
            if self.rest_cycles_in_line > 0 {
                let pos_ofs = Pos(self.next_render_x, self.next_render_y);
                self.render(&pos_ofs, self.rest_cycles_in_line, rom);
            }

            // 次のラインに進める.
            self.next_render_y += 1;
            if self.next_render_y >= 262 {
                // ライン0へ折り返す.
                self.next_render_y = 0;
                // フレームが完成.
                self.frame_count += 1;
            }
            let new_y = self.next_render_y;

            // 新ラインの初期化処理を行う.
            {
                // 0-239 line: visible scanline
                // ライン上のスプライトを探す
                if new_y <= 239 {
                    let mut count = 0;
                    for it in self.sprites.iter() {
                        let sy = it.y as u32;
                        if new_y <= sy && new_y + 7 >= sy {
                            self.line_sprites[count] = *it;
                            count += 1;
                            if count == 8 {
                                break;
                            }
                        }
                    }
                    self.line_sprite_count = count as u8;
                }

                // 240 line: post-render-scanline アイドル状態

                // 241-260 line: VBlank
                if new_y == 241 {
                    // start VBLANK
                    self.reg.status.vblank = true;
                    // NMI
                    if self.reg.ctrl & 0x80 != 0 {
                        self.nmi_interuption = true;
                    }
                }

                // 261 line: pre-render-scanling VBLANKフラグが下ろされる
                if new_y == 261 {
                    self.reg.status.vblank = false;
                }
            }

            // 新しいラインのレンダリング.
            let pos_ofs = Pos(0, new_y);
            self.render(&pos_ofs, excess, rom);

            self.next_render_x = excess;
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
                //assert!(self.reg.ctrl & 0x3 == 0, "not supported yet: {:b}", data);
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
                let addr = self.reg.oamaddr;
                let ith = addr as usize / 4;
                match addr % 4 {
                    0 => self.sprites[ith].y = data,
                    1 => self.sprites[ith].tile = data,
                    2 => self.sprites[ith].attr = data,
                    3 => self.sprites[ith].x = data,
                    _ => panic!("no way"),
                }
                self.reg.oamaddr += 1;
            }
            0x2005 => {
                if !self.reg.toggle_2005_2006 {
                    self.reg.scroll.0 = data;
                //println!("scroll.x: {}", data);
                } else {
                    self.reg.scroll.1 = data;
                    //println!("scroll.y: {}", data);
                }
                self.reg.toggle_2005_2006 = !self.reg.toggle_2005_2006;
            }
            0x2006 => {
                let w = u16::from(data);
                if !self.reg.toggle_2005_2006 {
                    self.reg.ppuaddr = (self.reg.ppuaddr & 0xff) | w << 8;
                } else {
                    self.reg.ppuaddr = (self.reg.ppuaddr & 0xff00) | w;
                }
                self.reg.toggle_2005_2006 = !self.reg.toggle_2005_2006;
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
                self.reg.toggle_2005_2006 = false;
                return r;
            }
            0x2007 => {
                let ppuaddr = self.reg.ppuaddr;
                let r = self.load_memory(ppuaddr);

                // アドレスのインクリメント
                let inc = if self.reg.ctrl & 0x4 == 0 { 1 } else { 32 };
                self.reg.ppuaddr += inc;
                return r;
            }
            _ => {
                unimplemented!("yet to implement ppu addr: 0x{:x}", addr);
            }
        }

        //panic!("yet to be implemented: {:x}", addr);
    }

    fn load_memory(&self, addr: u16) -> u8 {
        const NM0_LAST: u16 = NM0_BASE + NAME_SIZE - 1;
        const NM1_LAST: u16 = NM1_BASE + NAME_SIZE - 1;
        const NM2_LAST: u16 = NM2_BASE + NAME_SIZE - 1;
        const NM3_LAST: u16 = NM3_BASE + NAME_SIZE - 1;

        let ret = match addr {
            NM0_BASE..=NM0_LAST => self.name_table0[(addr - NM0_BASE) as usize],
            NM1_BASE..=NM1_LAST => self.name_table1[(addr - NM1_BASE) as usize],
            NM2_BASE..=NM2_LAST => self.name_table2[(addr - NM2_BASE) as usize],
            NM3_BASE..=NM3_LAST => self.name_table3[(addr - NM3_BASE) as usize],
            _ => unimplemented!(),
        };

        ret
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
            a if a >= NM0_BASE && a < NM0_BASE + NAME_SIZE => {
                let idx = (a - NM0_BASE) as usize;
                self.name_table0[idx] = data;
            }
            a if a >= AT0_BASE && a < AT0_BASE + ATTR_SIZE => {
                let idx = (a - AT0_BASE) as usize;
                self.attr_table0[idx] = data;
            }
            a if a >= NM1_BASE && a < NM1_BASE + NAME_SIZE => {
                let idx = (a - NM1_BASE) as usize;
                self.name_table1[idx] = data;
            }
            a if a >= AT1_BASE && a < AT1_BASE + ATTR_SIZE => {
                let idx = (a - AT1_BASE) as usize;
                self.attr_table1[idx] = data;
            }
            a if a >= NM2_BASE && a < NM2_BASE + NAME_SIZE => {
                let idx = (a - NM2_BASE) as usize;
                self.name_table2[idx] = data;
            }
            a if a >= AT2_BASE && a < AT2_BASE + ATTR_SIZE => {
                let idx = (a - AT2_BASE) as usize;
                self.attr_table2[idx] = data;
            }
            a if a >= NM3_BASE && a < NM3_BASE + NAME_SIZE => {
                let idx = (a - NM3_BASE) as usize;
                self.name_table3[idx] = data;
            }
            a if a >= AT3_BASE && a < AT3_BASE + ATTR_SIZE => {
                let idx = (a - AT3_BASE) as usize;
                self.attr_table3[idx] = data;
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
                unimplemented!("yet to be implemented to write addr: {:x}", addr);
            }
        }
    }

    pub fn do_oda_dma<T>(&mut self, src: &mut T) -> Cycle
    where
        T: std::io::Read + std::fmt::Debug,
    {
        let mut s = [0; 4];
        for sprite in self.sprites.iter_mut() {
            src.read(&mut s).unwrap();
            sprite.y = s[0];
            sprite.tile = s[1];
            sprite.attr = s[2];
            sprite.x = s[3];
        }

        514 // or 513 cycles
    }

    pub fn check_nmi_enabled(&mut self) -> bool {
        std::mem::replace(&mut self.nmi_interuption, false)
    }

    fn scroll_wrap(base: u32, scroll: u8, max: u32) -> (u32, bool) {
        let ap = base + scroll as u32;
        let p = ap % max;
        let mirror = ap > max;
        (p, mirror)
    }

    fn select_name_table(&self, x_mirror: bool, y_mirror: bool) -> (&Box<[u8]>, &Box<[u8]>) {
        match (x_mirror, y_mirror) {
            (false, false) => (&self.name_table0, &self.attr_table0),
            (true, false) => (&self.name_table1, &self.attr_table1),
            (false, true) => (&self.name_table2, &self.attr_table2),
            (true, true) => (&self.name_table3, &self.attr_table3),
        }
    }

    fn render(&mut self, pos: &Pos, pixel_count: u32, rom: &Rom) {
        if pos.0 >= WIDTH || pos.1 >= HEIGHT {
            return; // out of screen.
        }

        // https://wiki.nesdev.com/w/index.php/PPU_registers#PPUCTRL

        let bg_palette = &self.bg_palette;
        let sprite_palette = &self.sprite_palette;
        let chr_table = &rom.get_chr();

        let nwidth = std::cmp::min(pixel_count, WIDTH - pos.0);
        let mut out_color_indices = [0; WIDTH as usize];

        {
            let (scroll_x, scroll_y) = self.reg.scroll;
            let (y, y_mirror) = Self::scroll_wrap(pos.1, scroll_y, HEIGHT);
            let name_idx_in_line = (y / 8) * 32; // 8ピクセル毎、横に32個
            let attr_idx_in_line = (y / 32) * 8; // 32ピクセル毎、横に8個

            // 背景の処理
            for ix in 0..nwidth {
                let (x, x_mirror) = Self::scroll_wrap(pos.0 + ix, scroll_x, WIDTH);

                let name_idx = (name_idx_in_line + x / 8) as usize;
                let (name_table, attr_table) = self.select_name_table(x_mirror, y_mirror);
                let pat_idx = name_table[name_idx] as usize;

                // 背景の場合は chr_table にアクセス
                let pat_ofs = pat_idx * 16;
                let chr = &chr_table[pat_ofs..pat_ofs + 16];
                let pos_in_pat = Pos(x % 8, y % 8);
                let palette_idx: u8 = access_pat(chr, pos_in_pat, false); // [0, 3]

                // アトリビュートの取り出し
                let attr_idx = x / 32 + attr_idx_in_line;
                let attr = access_attr(attr_table[attr_idx as usize], pos);

                let pal_ofs = 4 * attr as usize;
                let col_idx = bg_palette[pal_ofs + palette_idx as usize];
                out_color_indices[ix as usize] = col_idx;
            }
        }

        // スプライト
        let spr_pat_tbl_base;
        {
            let ctrl = self.reg.ctrl;
            assert_eq!(ctrl & (0x1 << 5), 0, "16x8 spr is not implemented yet");
            let csf = ctrl & 0x08 != 0;
            spr_pat_tbl_base = if csf { 0x1000 } else { 0x0000 };
        }

        let y = pos.1;
        let mut found_v_mir = false;

        for ix in 0..nwidth {
            let x = pos.0 + ix;
            let ns = self.line_sprite_count as usize;
            for sprite in &self.line_sprites[0..ns] {
                let sx = sprite.x as u32;
                if x >= sx && x < sx + 8 {
                    let sy = sprite.y as u32;
                    let attr = sprite.attr;
                    let h_mir = attr & 0x40 != 0; // 左右反転
                    let v_mir = attr & 0x80 != 0; // 上下反転
                    found_v_mir |= v_mir; // 未実装チェック
                    let tile = sprite.tile as usize;
                    // スプライトも chr_table にアクセス
                    let pat_ofs = spr_pat_tbl_base + tile * 16;
                    let chr = &chr_table[pat_ofs..pat_ofs + 16];
                    let pos_in_pat = Pos(x - sx, 7 - (sy - y));
                    let pal_idx: u8 = access_pat(chr, pos_in_pat, h_mir); // [0, 3]
                    let pal_quad = attr & 0x3; // 2bitsのパレット取り出す
                    let pal_ofs = 4 * pal_quad as usize;
                    let col_idx = sprite_palette[pal_ofs + pal_idx as usize];
                    out_color_indices[ix as usize] = col_idx;
                }
            }
        }

        assert!(!found_v_mir, "not impl");

        // スプライトと背景の合成
        for ix in 0..nwidth {
            let x = pos.0 + ix;
            let col_idx = out_color_indices[ix as usize];
            let rgb = COLOR_PALETTE[col_idx as usize];
            self.frame_buffer.set_pixel(&Pos(x, y), &rgb);
        }
    }

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

    #[test]
    fn test_pos() {
        let rom = Rom::load_image("static/sample1/sample1.nes");

        {
            let mut ppu = PpuUnit::new();
            assert_eq!(ppu.get_next_render_pos(), Pos(0, 0));
            ppu.execute(0, &rom);
            assert_eq!(ppu.get_next_render_pos(), Pos(0, 0));
        }

        {
            let mut ppu = PpuUnit::new();
            ppu.execute(1, &rom);
            assert_eq!(ppu.get_next_render_pos(), Pos(1, 0));
        }

        {
            let mut ppu = PpuUnit::new();
            ppu.execute(CYCLES_PER_LINE, &rom);
            assert_eq!(ppu.get_next_render_pos(), Pos(0, 1));
        }
    }

    #[test]
    fn test_vblank0() {
        let rom = Rom::load_image("static/sample1/sample1.nes");
        let mut ppu = PpuUnit::new();
        let mut line_count = 0;
        loop {
            ppu.execute(CYCLES_PER_LINE, &rom);
            line_count += 1;
            if ppu.reg.status.vblank {
                break;
            }
        }
        assert_eq!(line_count, 241);
    }

    #[test]
    //#[ignore]
    fn test_vblank1() {
        let rom = Rom::load_image("static/sample1/sample1.nes");
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

        // この時点で最終ピクセルに到達
        assert_eq!(ppu.get_next_render_pos(), Pos(0, 0));
    }
}
