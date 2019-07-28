#![allow(dead_code)]

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
}

impl PpuRegister {}

pub struct PpuUnit {
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

    pub fn get_ppu_register(&self) -> &PpuRegister {
        &self.register
    }

    pub fn store_ppu_register(&mut self, addr: u16, data: u8) {
        //println!("xxx store_ppu_register: addr: {}, data: {}", addr, data);

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

    pub fn load_byte(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => {
                let mut r = 0;
                if self.register.status.vblank {
                    r |= 0x80
                }
                if self.register.status.sprite_hit {
                    r |= 0x40
                }

                self.register.status.vblank = false;

                self.register.toggle_ppuaddr = false;
                self.register.toggle_ppuscroll = false;
                return r;
            }
            _ => {}
        }

        panic!("yet to be implemented: {:x}", addr);
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
