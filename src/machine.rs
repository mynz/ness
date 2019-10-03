#![allow(dead_code)]

// https://www.masswerk.at/6502/6502_instruction_set.html

mod inst_specs;
mod tests;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

use self::inst_specs::INST_SPECS;
use crate::frame_buffer::FrameBuffer;
use crate::ppu::PpuUnit;
use crate::rom::Rom;
use crate::Cycle;

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
}

fn u8_to_i16(u: u8) -> i16 {
    u8_to_i8(u) as i16
}

fn pack_u16(b0: u8, b1: u8) -> u16 {
    let buf = [b0, b1];
    LittleEndian::read_u16(&buf)
}

fn unpack_u16(w: u16) -> (u8, u8) {
    let mut buf = [0_u8; 2];
    LittleEndian::write_u16(&mut buf, w);
    (buf[0], buf[1])
}

fn add_rel_to_pc(pc: u16, r: u8) -> u16 {
    let base = pc as i16;
    let rr = u8_to_i8(r) as i16;
    (base + rr) as u16
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operand {
    Implied,
    Accumulator,
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

#[derive(Debug)]
enum ExtCycles {
    Zero,
    One,
    OneOrTwo,
}

#[derive(Debug)]
struct InstSpec {
    code: u8,
    opcode: Opcode,
    operand: Operand,
    size: u8,
    cycles: u8,
    ext_cycles: ExtCycles,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Inst {
    pc: u16, // プログラムカウンタの位置
    code: u8,
    opcode: Opcode,
    operand: Operand,
}

impl Inst {
    fn decode<T: std::io::Read>(cur: &mut T, spec: &InstSpec, pc: u16) -> Self {
        //let n = (spec.size - 1) as usize;

        let operand = match spec.operand {
            Operand::Implied => Operand::Implied,
            Operand::Accumulator => Operand::Accumulator,
            Operand::Immediate(_u8) => Operand::Immediate(cur.read_u8().unwrap()),
            Operand::ZeroPage(_u8) => Operand::ZeroPage(cur.read_u8().unwrap()),
            Operand::ZeroPageX(_u8) => Operand::ZeroPageX(cur.read_u8().unwrap()),
            Operand::ZeroPageY(_u8) => Operand::ZeroPageY(cur.read_u8().unwrap()),
            Operand::Absolute(_u16) => Operand::Absolute(cur.read_u16::<LittleEndian>().unwrap()),
            Operand::AbsoluteX(_u16) => Operand::AbsoluteX(cur.read_u16::<LittleEndian>().unwrap()),
            Operand::AbsoluteY(_u16) => Operand::AbsoluteY(cur.read_u16::<LittleEndian>().unwrap()),
            Operand::Relative(_u8) => Operand::Relative(cur.read_u8().unwrap()),
            Operand::Indirect(_u16) => Operand::Indirect(cur.read_u16::<LittleEndian>().unwrap()),
            Operand::IndirectX(_u8) => Operand::IndirectX(cur.read_u8().unwrap()),
            Operand::IndirectY(_u8) => Operand::IndirectY(cur.read_u8().unwrap()),
        };

        Inst {
            pc: pc,
            code: spec.code,
            opcode: spec.opcode,
            operand,
        }
    }
}

impl Default for Inst {
    fn default() -> Inst {
        Inst {
            pc: 0x0000,
            code: 255,
            opcode: Opcode::NOP,
            operand: Operand::Implied,
        }
    }
}

// carry / overflow の違いは以下が分かりやすい
// https://lipoyang.hatenablog.com/entry/20131031/p1

struct StatusRegister {
    negative: bool,  // N
    overflow: bool,  // V
    b4: bool,        // B (bit 4)
    b5: bool,        // B (bit 5)
    decimal: bool,   // D
    interrupt: bool, // I
    zero: bool,      // Z
    carry: bool,     // C
}

impl StatusRegister {
    fn encode(&self) -> u8 {
        let v = if self.negative { 1 << 7 } else { 0 }
            | if self.overflow { 1 << 6 } else { 0 }
            | if self.b5 { 1 << 5 } else { 0 }
            | if self.b4 { 1 << 4 } else { 0 }
            | if self.decimal { 1 << 3 } else { 0 }
            | if self.interrupt { 1 << 2 } else { 0 }
            | if self.zero { 1 << 1 } else { 0 }
            | if self.carry { 1 << 0 } else { 0 };
        v
    }

    fn decode(v: u8) -> StatusRegister {
        Self {
            negative: v & (1 << 7) != 0,
            overflow: v & (1 << 6) != 0,
            b5: v & (1 << 5) != 0,
            b4: v & (1 << 4) != 0,
            decimal: v & (1 << 3) != 0,
            interrupt: v & (1 << 2) != 0,
            zero: v & (1 << 1) != 0,
            carry: v & (1 << 0) != 0,
        }
    }
}

impl Default for StatusRegister {
    fn default() -> Self {
        StatusRegister {
            negative: false,
            overflow: false,
            b4: false, // break
            b5: true,  // break
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }
}

struct Register {
    a: u8, // accumelater
    x: u8, // index register
    y: u8, // index register
    s: u8, // stack pointer
    p: StatusRegister,
    pc: u16,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0xfd,
            p: StatusRegister::default(),
            pc: 0xc000,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.a,
            self.x,
            self.y,
            self.p.encode(),
            self.s
        )
    }
}

#[derive(Default)]
struct Joypad {
    keybits: u32,
    load_count: u8,
    store_count: u8,
}

impl Joypad {
    pub fn set_joypad_keybits(&mut self, keybits: u32) {
        self.keybits = keybits;
    }

    fn load_byte(&mut self) -> u8 {
        let c = self.load_count;
        let ret = (self.keybits >> c) as u8 & 0x1;
        self.load_count += 1;
        ret
    }
    fn store_byte(&mut self, data: u8) {
        if self.store_count == 1 && data == 0 {
            // reset
            self.load_count = 0;
        }
        self.store_count = data;
    }
}

#[derive(Default)]
pub struct Executer {
    register: Register,
    wram: Box<[u8]>, // 2kb
    ppu_unit: PpuUnit,
    rom: Box<Rom>,
    joypad0: Joypad,
    joypad1: Joypad,

    last_exec_inst: Inst,
    cycles: Cycle,
}

impl Executer {
    pub fn new() -> Self {
        let wram = Box::new([0u8; 0x800]);
        Self {
            wram,
            ..Default::default()
        }
    }

    pub fn set_rom(&mut self, rom: Rom) {
        self.rom = Box::new(rom);
    }

    pub fn set_joypad_keybits(&mut self, pad_idx: u8, keybits: u32) {
        let pad = if pad_idx == 0 {
            &mut self.joypad0
        } else {
            &mut self.joypad1
        };
        pad.set_joypad_keybits(keybits);
    }

    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr <= 0x07ff {
            return self.wram[addr as usize];
        } else if addr >= 0x2000 && addr < 0x4000 {
            return self.ppu_unit.load_byte(addr);
        } else if addr == 0x4016 {
            return self.joypad0.load_byte(); // joypad0
        } else if addr == 0x4017 {
            return self.joypad1.load_byte(); // joypad1
        } else if addr >= 0x8000 {
            let base = 0x8000;
            let prg_size = self.rom.get_prg().len();
            let mut ofs = (addr - base) as u64;

            // prg が 16kb しかない場合は、0xc000 からの領域にミラーリングされていると見なす
            if ofs >= 0x4000 && prg_size <= 0x4000 {
                ofs -= 0x4000;
            }

            let mut cur = Cursor::new(self.rom.get_prg());
            cur.set_position(ofs);
            return cur.read_u8().unwrap();
        }

        unimplemented!("load_word: {:x}", addr);
    }

    fn load_word(&mut self, addr: u16) -> u16 {
        let d0 = self.load_byte(addr) as u16;
        let d1 = self.load_byte(addr + 1) as u16;
        (d1 << 8 | d0) as u16
    }

    fn load_word_in_page(&mut self, addr: u16) -> u16 {
        let a0 = addr;
        let a1 = {
            let h = addr & 0xff00;
            let l = addr & 0x00ff;
            h + ((l + 1) & 0xff)
        };

        let d0 = self.load_byte(a0) as u16;
        let d1 = self.load_byte(a1) as u16;
        (d1 << 8 | d0) as u16
    }

    fn load_word_zero_paged(&mut self, addr: u16) -> u16 {
        let zp = |a| a & 0x00ff;

        let d0 = self.load_byte(zp(addr)) as u16;
        let d1 = self.load_byte(zp(addr + 1)) as u16;
        (d1 << 8 | d0) as u16
    }

    fn store_byte(&mut self, addr: u16, data: u8) -> Cycle {
        if addr < 0x0800 {
            self.wram[addr as usize] = data;
        } else if addr >= 0x2000 && addr < 0x2008 {
            self.ppu_unit.store_from_cpu(addr, data);
        } else if addr >= 0x4000 && addr <= 0x4013 {
            // TODO: APU
        } else if addr == 0x4015 {
            // TODO: APU
        } else if addr == 0x4014 {
            // ODA DMA
            let addr_from = 0x100 * data as usize;
            if addr_from < 0x800 {
                let addr_to = addr_from + 0x100;
                let mut src = Cursor::new(&self.wram[addr_from..addr_to]);
                return self.ppu_unit.do_oda_dma(&mut src);
            }
            unimplemented!();
        } else if addr == 0x4016 {
            // joypad0
            self.joypad0.store_byte(data);
            self.joypad1.store_byte(data);
        } else if addr == 0x4017 {
            // TODO: APU frame count(frame sequencer)
        } else {
            panic!("store_byte out of bound: {:x}, {:x}", addr, data);
        };

        0 // 指定がなければ zero cycles
    }

    pub fn hard_reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.load_word(0xfffc);
    }

    fn fetch_inst(&mut self) -> (Inst, &'static InstSpec) {
        let pc = self.register.pc;
        let op = self.load_byte(pc);
        self.register.pc += 1;

        let inst_spec = &INST_SPECS[op as usize];

        let mut bytes = [0_u8; 2];
        {
            let nrest = inst_spec.size - 1;
            if nrest == 1 {
                let b = self.load_byte(self.register.pc);
                bytes[0] = b;
            } else if nrest == 2 {
                let w = self.load_word(self.register.pc);
                bytes.as_mut().write_u16::<LittleEndian>(w).unwrap();
            };

            self.register.pc += nrest as u16;
        }

        (Inst::decode(&mut bytes.as_ref(), inst_spec, pc), inst_spec)
    }

    fn get_addr_from_operand(&mut self, operand: Operand) -> u16 {
        let addr: u16 = match operand {
            Operand::Immediate(_) => unreachable!("must not come Immediate"),
            Operand::ZeroPage(a) => a as u16,
            Operand::ZeroPageX(a) => (a as u16 + self.register.x as u16) & 0xff,
            Operand::ZeroPageY(a) => (a as u16 + self.register.y as u16) & 0xff,
            Operand::Absolute(a) => a,
            Operand::AbsoluteX(a) => (a as i16 + u8_to_i16(self.register.x)) as u16,
            Operand::AbsoluteY(a) => (a as i16 + u8_to_i16(self.register.y)) as u16,
            Operand::IndirectX(a) => {
                self.load_word_zero_paged((a as u16 + self.register.x as u16) & 0xff)
            }
            Operand::IndirectY(a) => {
                let m = self.load_word_zero_paged(a as u16);
                let y = self.register.y as u16;
                (m as u32 + y as u32) as u16
            }
            _ => panic!("no impl: {:#?}", operand),
        };
        addr
    }

    fn execute_inst(&mut self, inst: &Inst) -> Cycle {
        let mut extra_cycles = 0;

        match inst.opcode {
            Opcode::ADC | Opcode::SBC => {
                // http://taotao54321.hatenablog.com/entry/2017/04/09/151355

                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => {
                        let addr = self.get_addr_from_operand(inst.operand);
                        self.load_byte(addr)
                    }
                };

                // SBCはADCの引数を反転させることで実現できる
                // https://stackoverflow.com/a/29224684
                let b: u16 = if inst.opcode == Opcode::SBC { !m } else { m } as u16;
                let a: u16 = self.register.a as u16;
                let c: u16 = if self.register.p.carry { 1 } else { 0 };

                let d16 = a + b + c;
                let c = d16 > 0x00ff;
                let d = d16 as u8;

                // V: 符号付きオーバーフローが発生したか
                // (同符号の値を加算した結果符号が変わったら真)
                let v = {
                    let s0 = a >> 7 & 0x01;
                    let s1 = b >> 7 & 0x01;
                    let s2 = d >> 7 & 0x01;
                    s0 == s1 && s0 != s2.into()
                };

                self.register.a = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.carry = c;
                self.register.p.overflow = v;
            }
            Opcode::AND | Opcode::EOR | Opcode::ORA => {
                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => {
                        let addr = self.get_addr_from_operand(inst.operand);
                        self.load_byte(addr)
                    }
                };

                let d = match inst.opcode {
                    Opcode::AND => self.register.a & m,
                    Opcode::EOR => self.register.a ^ m,
                    Opcode::ORA => self.register.a | m,
                    _ => unreachable!(),
                };

                self.register.a = d;
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.zero = d == 0;
            }
            Opcode::ASL | Opcode::LSR | Opcode::ROL | Opcode::ROR => {
                // 対象はメモリだけでなく、Aレジスタの場合もある

                enum T {
                    RegA,
                    Mem(u16),
                };

                let (t, s) = match inst.operand {
                    Operand::Accumulator => (T::RegA, self.register.a),
                    _ => {
                        let addr = self.get_addr_from_operand(inst.operand);
                        (T::Mem(addr), self.load_byte(addr))
                    }
                };

                let (d, c) = match inst.opcode {
                    Opcode::ASL => (s << 1, s & 0x80 != 0),
                    Opcode::LSR => (s >> 1, s & 0x01 != 0),
                    Opcode::ROL => {
                        let c = if self.register.p.carry { 1 } else { 0 };
                        (s << 1 | c, s & 0x80 != 0)
                    }
                    Opcode::ROR => {
                        let c = if self.register.p.carry { 0x80 } else { 0 };
                        (s >> 1 | c, s & 0x01 != 0)
                    }
                    _ => unreachable!(),
                };

                if let T::Mem(addr) = t {
                    self.store_byte(addr, d);
                } else {
                    self.register.a = d;
                }

                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.carry = c;
            }
            Opcode::BCC
            | Opcode::BCS
            | Opcode::BEQ
            | Opcode::BNE
            | Opcode::BMI
            | Opcode::BPL
            | Opcode::BVC
            | Opcode::BVS => {
                let cond = match inst.opcode {
                    Opcode::BCC => !self.register.p.carry,
                    Opcode::BCS => self.register.p.carry,
                    Opcode::BEQ => self.register.p.zero,
                    Opcode::BNE => !self.register.p.zero,
                    Opcode::BMI => self.register.p.negative,
                    Opcode::BPL => !self.register.p.negative,
                    Opcode::BVC => !self.register.p.overflow,
                    Opcode::BVS => self.register.p.overflow,
                    _ => unreachable!(),
                };

                if cond {
                    let r = match inst.operand {
                        Operand::Relative(r) => r,
                        _ => unreachable!(),
                    };
                    self.register.pc = add_rel_to_pc(self.register.pc, r);
                }
            }
            Opcode::BIT => {
                let addr = self.get_addr_from_operand(inst.operand);
                let m = self.load_byte(addr);
                let a = self.register.a;

                let d = a & m;
                let z = d == 0;
                let n = (m & (1 << 7)) != 0;
                let v = (m & (1 << 6)) != 0;

                self.register.p.negative = n;
                self.register.p.zero = z;
                self.register.p.overflow = v;
            }
            Opcode::CLC => {
                self.register.p.carry = false;
            }
            Opcode::CLD => {
                self.register.p.decimal = false;
            }
            Opcode::CLV => {
                self.register.p.overflow = false;
            }
            Opcode::CMP | Opcode::CPX | Opcode::CPY => {
                // http://www.6502.org/tutorials/compare_beyond.html#2.1
                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => {
                        let addr = self.get_addr_from_operand(inst.operand);
                        self.load_byte(addr)
                    }
                };

                let s = match inst.opcode {
                    Opcode::CMP => self.register.a,
                    Opcode::CPX => self.register.x,
                    Opcode::CPY => self.register.y,
                    _ => unreachable!(),
                };

                let n = (s.wrapping_sub(m) & 0x80) != 0;

                self.register.p.negative = n;
                self.register.p.zero = s == m;
                self.register.p.carry = s >= m;
            }
            Opcode::DEC => {
                let addr = self.get_addr_from_operand(inst.operand);

                let s = self.load_byte(addr);
                let d = if s == 0 { 0xff } else { s - 1 };
                self.store_byte(addr, d);

                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::DEX | Opcode::DEY => {
                let reg = match inst.opcode {
                    Opcode::DEX => &mut self.register.x,
                    Opcode::DEY => &mut self.register.y,
                    _ => unreachable!(),
                };
                let d = if *reg == 0 { 0xff } else { *reg - 1 };
                *reg = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::INC => {
                let addr = self.get_addr_from_operand(inst.operand);
                let s = self.load_byte(addr);
                let d = if s == 0xff { 0 } else { s + 1 };
                self.store_byte(addr, d);

                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::INX | Opcode::INY => {
                let r = match inst.opcode {
                    Opcode::INX => &mut self.register.x,
                    Opcode::INY => &mut self.register.y,
                    _ => unreachable!(),
                };
                let s = *r;
                let d = if s == 0xff { 0 } else { s + 1 };
                *r = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::JMP => {
                let d = match inst.operand {
                    Operand::Absolute(v) => v,
                    Operand::Indirect(v) => self.load_word_in_page(v),
                    _ => unimplemented!(),
                };
                self.register.pc = d;
            }
            Opcode::JSR => {
                let d = match inst.operand {
                    Operand::Absolute(v) => v,
                    _ => unreachable!(),
                };

                self.push_u16(inst.pc + 2);
                self.register.pc = d;
            }
            Opcode::LDA | Opcode::LDX | Opcode::LDY => {
                let d: u8 = if let Operand::Immediate(v) = inst.operand {
                    v
                } else {
                    let addr = self.get_addr_from_operand(inst.operand);
                    self.load_byte(addr)
                };

                let r = match inst.opcode {
                    Opcode::LDA => &mut self.register.a,
                    Opcode::LDX => &mut self.register.x,
                    Opcode::LDY => &mut self.register.y,
                    _ => unreachable!(),
                };
                *r = d;

                self.register.p.negative = d & 0x80 != 0;
                self.register.p.zero = d == 0;
            }
            Opcode::NOP => {
                debug_assert_eq!(inst.operand, Operand::Implied);
            }
            Opcode::PHA => {
                self.push_u8(self.register.a);
            }
            Opcode::PHP => {
                // https://wiki.nesdev.com/w/index.php/Status_flags
                // b4 はスペック的には true であるべきだが nestest.log は false の挙動なのか？
                //self.register.p.b4 = true; // questionable
                //self.register.p.b5 = true; // okay
                let d = self.register.p.encode();
                self.push_u8(d);
            }
            Opcode::PLA => {
                debug_assert_eq!(inst.operand, Operand::Implied);
                let d = self.pop_u8();
                self.register.a = d;
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.zero = d == 0;
            }
            Opcode::PLP | Opcode::RTI => {
                // bit4, bit5 は無視する挙動
                // https://wiki.nesdev.com/w/index.php/Status_flags

                let d = self.pop_u8();
                let o = self.register.p.encode();
                let p = (0x30 & o) | (0xcf & d);
                self.register.p = StatusRegister::decode(p);

                if inst.opcode == Opcode::RTI {
                    self.register.pc = self.pop_u16();
                }
            }
            Opcode::RTS => {
                let v = self.pop_u16();
                self.register.pc = v + 1;
            }
            Opcode::SEC => {
                self.register.p.carry = true;
            }
            Opcode::SED => {
                self.register.p.decimal = true;
            }
            Opcode::SEI => {
                self.register.p.interrupt = true;
            }
            Opcode::STA => {
                let addr = self.get_addr_from_operand(inst.operand);
                extra_cycles = self.store_byte(addr, self.register.a);
            }
            Opcode::STX | Opcode::STY => {
                let addr = self.get_addr_from_operand(inst.operand);
                let s = match inst.opcode {
                    Opcode::STX => self.register.x,
                    Opcode::STY => self.register.y,
                    _ => panic!(),
                };
                extra_cycles = self.store_byte(addr, s);
            }
            Opcode::TAX | Opcode::TAY | Opcode::TSX | Opcode::TXA | Opcode::TYA => {
                let d: u8 = match inst.opcode {
                    Opcode::TAX => {
                        self.register.x = self.register.a;
                        self.register.x
                    }
                    Opcode::TAY => {
                        self.register.y = self.register.a;
                        self.register.y
                    }
                    Opcode::TSX => {
                        self.register.x = self.register.s;
                        self.register.x
                    }
                    Opcode::TXA => {
                        self.register.a = self.register.x;
                        self.register.a
                    }
                    Opcode::TYA => {
                        self.register.a = self.register.y;
                        self.register.a
                    }
                    _ => unreachable!(),
                };
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.zero = d == 0;
            }
            Opcode::TXS => {
                // 他のT*シリーズと違ってPフラグを操作しない
                self.register.s = self.register.x;
            }
            _ => {
                unimplemented!("yet to not impl: {:x?}", inst);
            }
        }

        self.last_exec_inst = *inst;

        extra_cycles
    }

    fn push_u8(&mut self, v: u8) {
        let stack_base = 0x0100;

        let s = self.register.s as u16 + stack_base;
        self.store_byte(s, v);
        self.register.s = self.register.s.wrapping_sub(1);
    }

    fn pop_u8(&mut self) -> u8 {
        let stack_base = 0x0100;

        self.register.s = self.register.s.wrapping_add(1);
        let s = self.register.s as u16 + stack_base;
        let v = self.load_byte(s);
        v
    }

    fn push_u16(&mut self, v: u16) {
        // スタックレジスタはラップする。
        // https://superuser.com/questions/346658/does-the-6502-put-ff-in-the-stack-pointer-register-as-soon-as-it-gets-power-for

        let (v0, v1) = unpack_u16(v);
        let stack_base = 0x0100;

        let s = self.register.s as u16 + stack_base;
        self.store_byte(s, v1);
        self.register.s = self.register.s.wrapping_sub(1);

        let s = self.register.s as u16 + stack_base;
        self.store_byte(s, v0);
        self.register.s = self.register.s.wrapping_sub(1);
    }

    fn pop_u16(&mut self) -> u16 {
        let stack_base = 0x0100;

        self.register.s = self.register.s.wrapping_add(1);
        let s = self.register.s as u16 + stack_base;
        let v0 = self.load_byte(s);

        self.register.s = self.register.s.wrapping_add(1);
        let s = self.register.s as u16 + stack_base;
        let v1 = self.load_byte(s);

        let v = pack_u16(v0, v1);
        v
    }

    pub fn execute(&mut self) -> u8 {
        // NMIの検出
        if self.ppu_unit.check_nmi_enabled() {
            self.register.p.b4 = false;
            self.register.p.b5 = true;
            self.push_u16(self.register.pc);
            self.push_u8(self.register.p.encode());
            self.register.p.interrupt = true;
            self.register.pc = self.load_word(0xfffa);

            //println!(
            //"check_nmi_enabled: {:?}",
            //(self.get_frame_count(), self.cycles, self.register.pc)
            //);
        }

        let (inst, spec) = self.fetch_inst();

        if false {
            //println!("xxx: {:X?}, {}", inst, self.register);
            println!("xxx: {:X}, {:?} {}", inst.pc, inst.opcode, self.register);
        }

        // DMA用
        let extra_cycles = self.execute_inst(&inst);

        // PPUは3倍で進む
        let cycles_delta = spec.cycles as Cycle + extra_cycles;
        let ppu_cycles = 3 * cycles_delta;
        self.ppu_unit.execute(ppu_cycles, &self.rom);

        self.cycles += cycles_delta;
        spec.cycles
    }

    pub fn get_frame_count(&self) -> u32 {
        self.ppu_unit.get_frame_count()
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        self.ppu_unit.get_frame_buffer()
    }
}
