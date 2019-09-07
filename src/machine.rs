#![allow(dead_code)]

// https://www.masswerk.at/6502/6502_instruction_set.html

mod inst_specs;
mod tests;

use byteorder::{LittleEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
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

    pub fn set_joypad_keybits(&mut self, keybits: u32) {
        self.joypad0.set_joypad_keybits(keybits);
    }

    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr >= 0x2000 && addr < 0x4000 {
            return self.ppu_unit.load_byte(addr);
        } else if addr == 0x4016 {
            // joypad0
            return self.joypad0.load_byte();
        }

        let word = self.load_word(addr);
        (word & 0x00ff) as u8
    }

    fn load_word(&mut self, addr: u16) -> u16 {
        if addr <= 0x07ff {
            let mut cur = Cursor::new(&self.wram);
            cur.set_position(addr as u64);
            return cur.read_u16::<LittleEndian>().unwrap();
        }

        if addr >= 0x2000 && addr < 0x4000 {
            return self.ppu_unit.load_word(addr);
        }

        // 0xffff まで有効
        if addr >= 0x8000 {
            let base = 0x8000;
            let prg_size = self.rom.get_prg().len();
            let mut ofs = (addr - base) as u64;

            // prg が 16kb しかない場合は、0xc000 からの領域にミラーリングされていると見なす
            if ofs >= 0x4000 && prg_size <= 0x4000 {
                ofs -= 0x4000;
            }

            let mut cur = Cursor::new(self.rom.get_prg());
            cur.set_position(ofs);
            return cur.read_u16::<LittleEndian>().unwrap();
        }

        panic!("yet to be implemented: {:x}", addr);
    }

    fn store_byte(&mut self, addr: u16, data: u8) -> Cycle {
        if addr < 0x0800 {
            self.wram[addr as usize] = data;
        } else if addr >= 0x2000 && addr < 0x2008 {
            self.ppu_unit.store_from_cpu(addr, data);
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

    fn execute_inst(&mut self, inst: &Inst) -> Cycle {
        let mut extra_cycles = 0;

        match inst.opcode {
            Opcode::ADC => {
                // http://taotao54321.hatenablog.com/entry/2017/04/09/151355

                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => unimplemented!(),
                };

                let a = self.register.a;
                let c = if self.register.p.carry { 1 } else { 0 };
                let d16 = (a + m + c) as u16;
                let c = d16 > 0x00ff;
                let d = d16 as u8;

                // V: 符号付きオーバーフローが発生したか(同符号の値を加算した結果符号が変わったら真)
                let v = {
                    let s0 = a >> 7;
                    let s1 = m >> 7;
                    let s2 = d >> 7;
                    s0 == s1 && s0 != s2
                };

                self.register.a = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
                self.register.p.carry = c;
                self.register.p.overflow = v;
            }
            Opcode::AND => {
                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => unimplemented!(),
                };
                let d = self.register.a & m;
                self.register.a = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::ASL | Opcode::LSR => {
                // 対象はメモリだけでなく、Aレジスタの場合もある
                let m = match inst.operand {
                    Operand::ZeroPage(v) => v,
                    Operand::Accumulator => unimplemented!("A regiser"),
                    _ => unimplemented!(),
                } as u16;

                let s = self.load_byte(m);

                let (d, c) = match inst.opcode {
                    Opcode::ASL => (s << 1, s & 0x80 != 0),
                    Opcode::LSR => (s >> 1, s & 0x01 != 0),
                    _ => unreachable!(),
                };

                self.store_byte(m, d);

                let n = inst.opcode == Opcode::ASL && (d & 0x80 != 0);

                self.register.p.zero = d == 0;
                self.register.p.negative = n;
                self.register.p.carry = c;
            }
            Opcode::BNE => {
                if !self.register.p.zero {
                    let r = match inst.operand {
                        Operand::Relative(r) => r,
                        _ => unreachable!(),
                    };
                    self.register.pc = add_rel_to_pc(self.register.pc, r);
                }
            }
            Opcode::BPL => {
                if !self.register.p.negative {
                    let r = match inst.operand {
                        Operand::Relative(v) => v,
                        _ => unreachable!(),
                    };
                    self.register.pc = add_rel_to_pc(self.register.pc, r);
                }
            }
            Opcode::CPX | Opcode::CPY => {
                let m = match inst.operand {
                    Operand::Immediate(v) => v,
                    Operand::ZeroPage(v) => self.load_byte(v as u16),
                    Operand::Absolute(v) => self.load_byte(v),
                    _ => unreachable!(),
                };

                let s = match inst.opcode {
                    Opcode::CPX => self.register.x,
                    Opcode::CPY => self.register.y,
                    _ => unreachable!(),
                };

                self.register.p.zero = s == m;
                self.register.p.negative = s < m;
                self.register.p.carry = s >= m;
            }
            Opcode::DEC => {
                let addr = match inst.operand {
                    Operand::Absolute(v) => v,
                    _ => unreachable!(),
                };

                let s = self.load_byte(addr);
                let d = if s == 0 { 0xff } else { s - 1 };
                self.store_byte(addr, d);

                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::DEY => {
                let s = self.register.y;
                let d = if s == 0 { 0xff } else { s - 1 };
                self.register.y = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::INC => {
                let addr = match inst.operand {
                    Operand::Absolute(v) => v,
                    _ => unreachable!(),
                };

                let s = self.load_byte(addr);
                let d = if s == 0xff { 0 } else { s + 1 };
                self.store_byte(addr, d);

                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::INX => {
                let s = self.register.x;
                let d = if s == 0xff { 0 } else { s + 1 };
                self.register.x = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::JMP => {
                let d = match inst.operand {
                    Operand::Absolute(v) => v,
                    Operand::Indirect(v) => self.load_word(v),
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
            Opcode::LDA => {
                let d = match inst.operand {
                    Operand::Immediate(v) => v,
                    Operand::Absolute(addr) => self.load_byte(addr),
                    Operand::AbsoluteX(addr) => {
                        self.load_byte((addr as i16 + u8_to_i16(self.register.x)) as u16)
                    }
                    Operand::AbsoluteY(addr) => {
                        self.load_byte((addr as i16 + u8_to_i16(self.register.y)) as u16)
                    }
                    _ => panic!("no impl: {:#?}", inst.operand),
                };
                self.register.a = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::LDX => {
                let d = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => unimplemented!(),
                };
                self.register.x = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::LDY => {
                let d = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => unimplemented!(),
                };
                self.register.y = d;
                self.register.p.zero = d == 0;
                self.register.p.negative = d & 0x80 != 0;
            }
            Opcode::NOP => {
                assert_eq!(inst.operand, Operand::Implied);
            }
            Opcode::RTS => {
                let v = self.pop_u16();
                self.register.pc = v + 1;
            }
            Opcode::SEI => {
                self.register.p.interrupt = true;
            }
            Opcode::STA => {
                let addr = match inst.operand {
                    Operand::Absolute(a) => a,
                    _ => unimplemented!(),
                };
                extra_cycles = self.store_byte(addr, self.register.a);
            }
            Opcode::TXS => {
                self.register.s = self.register.x;
            }
            _ => {
                panic!("yet to not impl: {:?}", inst);
            }
        }

        self.last_exec_inst = *inst;

        extra_cycles
    }

    fn push_u16(&mut self, v: u16) {
        // スタックレジスタはラップする。
        // https://superuser.com/questions/346658/does-the-6502-put-ff-in-the-stack-pointer-register-as-soon-as-it-gets-power-for

        let (v0, v1) = unpack_u16(v);
        let stack_base = 0x0100;

        let s = self.register.s as u16 + stack_base;
        self.store_byte(s, v0);
        self.register.s = self.register.s.wrapping_sub(1);

        let s = self.register.s as u16 + stack_base;
        self.store_byte(s, v1);
        self.register.s = self.register.s.wrapping_sub(1);
    }

    fn pop_u16(&mut self) -> u16 {
        let stack_base = 0x0100;

        self.register.s = self.register.s.wrapping_add(1);
        let s = self.register.s as u16 + stack_base;
        let v1 = self.load_byte(s);

        self.register.s = self.register.s.wrapping_add(1);
        let s = self.register.s as u16 + stack_base;
        let v0 = self.load_byte(s);

        let v = pack_u16(v0, v1);
        v
    }

    pub fn execute(&mut self) -> u8 {
        // NMIの検出
        if self.ppu_unit.check_nmi_enabled() {
            self.register.p.brk = false;
            self.push_u16(self.register.pc);
            self.register.p.interrupt = true;
            self.register.pc = self.load_word(0xfffa);

            //println!(
                //"check_nmi_enabled: {:?}",
                //(self.get_frame_count(), self.cycles, self.register.pc)
            //);
        }

        let (inst, spec) = self.fetch_inst();
        //println!("xxx: inst {:#?}: ", inst);

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
