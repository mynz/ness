#![allow(dead_code)]

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
    pub fn new() -> PpuUnit {
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

    pub fn store_ppu_register(&mut self, addr: u16, data: u8) {
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
                self.register.toggle_ppuaddr = !self.register.toggle_ppuaddr;
            }
            7 => {
                let addr = self.register.ppuaddr;
                self.store_memory(addr, data);

                // アドレスのインクリメント
                let inc = if self.register.ctrl & 0x4 == 0 { 1 } else { 32 };
                self.register.ppuaddr += inc;
            }
            _ => {
                assert!(false, "yet to be implemented");
            }
        }
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
