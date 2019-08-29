#![allow(dead_code)]

mod inst_specs;
mod tests;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

use self::inst_specs::INST_SPECS;
use crate::ppu::PpuUnit;
use crate::rom::Rom;
use crate::frame_buffer::FrameBuffer;

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
}

fn u8_to_i16(u: u8) -> i16 {
    u8_to_i8(u) as i16
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
pub struct Executer {
    register: Register,
    wram: Box<[u8]>, // 2kb
    ppu_unit: PpuUnit,
    rom: Box<Rom>,

    last_exec_inst: Inst,
    cycles: u32,
}

impl Executer {
    pub fn new() -> Self {
        Executer::default()
    }

    pub fn set_rom(&mut self, rom: Rom) {
        self.rom = Box::new(rom);
    }

    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr >= 0x2000 && addr < 0x4000 {
            return self.ppu_unit.load_byte(addr);
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

        assert!(false, "yet to be implemented: {:x}", addr);
        return 0;
    }

    fn store_byte(&mut self, addr: u16, data: u8) {
        if addr >= 0x2000 && addr < 0x2008 {
            self.ppu_unit.store_from_cpu(addr, data);
            return;
        }
        assert!(false, "yet to be implemented");
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

    fn execute_inst(&mut self, inst: &Inst) {
        match inst.opcode {
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
            Opcode::DEY => {
                let s = self.register.y;
                let d = if s == 0 { 0xff } else { s - 1 };
                self.register.y = d;
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
            Opcode::SEI => {
                self.register.p.interrupt = true;
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
            Opcode::STA => {
                let addr = match inst.operand {
                    Operand::Absolute(a) => a,
                    _ => unimplemented!(),
                };
                self.store_byte(addr, self.register.a);
            }
            Opcode::TXS => {
                self.register.s = self.register.x;
            }
            _ => {
                panic!("yet to not impl: {:#?}", inst);
            }
        }

        self.last_exec_inst = *inst;
    }

    pub fn execute(&mut self) -> u8 {
        let (inst, spec) = self.fetch_inst();
        //println!("xxx: inst {:#?}: ", inst);
        self.execute_inst(&inst);

        // PPUは3倍で進む
        let cycles_delta = spec.cycles as u32;
        self.ppu_unit.execute(3 * cycles_delta, &self.rom);

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
