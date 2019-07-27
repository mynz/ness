#![allow(dead_code)]

mod inst_specs;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

use self::inst_specs::INST_SPECS;
use crate::ppu::PpuUnit;
use crate::rom::Rom;

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
}

impl Register {}

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
struct Executer {
    register: Register,
    wram: Box<[u8]>, // 2kb
    ppu_unit: PpuUnit,
    rom: Box<Rom>,

    last_exec_inst: Inst,
    cycles: u32,
}

impl Executer {
    fn new() -> Self {
        Executer::default()
    }

    fn set_rom(&mut self, rom: Box<Rom>) {
        self.rom = rom;
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        if addr >= 0x2000 && addr < 0x4000 {
            return self.ppu_unit.load_byte(addr);
        }

        let word = self.read_word(addr);
        (word & 0x00ff) as u8
    }

    fn read_word(&mut self, addr: u16) -> u16 {
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

    fn write_byte(&mut self, addr: u16, data: u8) {
        if addr >= 0x2000 && addr < 0x2008 {
            let a = addr - 0x2000;
            self.ppu_unit.store_ppu_register(a, data);
            return;
        }
        assert!(false, "yet to be implemented");
    }

    fn hard_reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.read_word(0xfffc);
    }

    fn fetch_inst(&mut self) -> (Inst, &'static InstSpec) {
        let pc = self.register.pc;
        let op = self.read_byte(pc);
        self.register.pc += 1;

        let inst_spec = &INST_SPECS[op as usize];

        let mut bytes = [0_u8; 2];
        {
            let nrest = inst_spec.size - 1;
            if nrest == 1 {
                let b = self.read_byte(self.register.pc);
                bytes[0] = b;
            } else if nrest == 2 {
                let w = self.read_word(self.register.pc);
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
                        _ => unimplemented!(),
                    };
                    let base = self.register.pc as i16;
                    let rr = u8_to_i8(r) as i16;
                    self.register.pc = (base + rr) as u16;
                }
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
                    Operand::Indirect(v) => self.read_word(v),
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
                    Operand::Absolute(addr) => self.read_byte(addr),
                    Operand::AbsoluteX(addr) => {
                        self.read_byte(addr + self.register.x as u16)
                    }
                    Operand::AbsoluteY(addr) => {
                        self.read_byte(addr + self.register.y as u16)
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
                self.write_byte(addr, self.register.a);
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

    fn execute(&mut self) -> u8 {
        let (inst, spec) = self.fetch_inst();
        //println!("xxx: inst {:#?}: ", inst);
        self.execute_inst(&inst);

        self.cycles += spec.cycles as u32;
        spec.cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv() {
        assert_eq!(u8_to_i8(0x00), 0);
        assert_eq!(u8_to_i8(0xff), -1);
        assert_eq!(u8_to_i8(0x7f), 0x7f);
        assert_eq!(u8_to_i8(0x80), -128);
    }

    #[test]
    fn test_inst() {
        for i in 0..256 {
            let inst_spec = &INST_SPECS[i];
            assert!(inst_spec.size > 0);
        }
        assert!(INST_SPECS[0].opcode == Opcode::BRK);
    }

    #[test]
    fn test_executer_01() {
        let mut exe = Executer::new();
        assert_eq!(0, exe.register.a);

        let rom = Rom::load_image("static/roms/giko005.nes".to_string());
        exe.set_rom(rom);

        exe.hard_reset();

        //let cycles_beg = exe.cycles;

        assert_eq!(exe.register.pc, 0x8000);
        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDX);

        println!("xxx: last_exec_inst: {:#?}", exe.last_exec_inst);
    }

    #[test]
    fn test_executer_00() {
        let mut exe = Executer::new();
        assert_eq!(0, exe.register.a);

        let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
        exe.set_rom(rom);

        exe.hard_reset();

        let cycles_beg = exe.cycles;

        assert_eq!(exe.register.pc, 0x8000);
        exe.execute();
        assert_eq!(exe.register.x, 0x00);
        exe.execute();
        assert_eq!(exe.register.x, 0xff);
        assert_eq!(exe.register.s, 0x00);
        exe.execute(); // TXS
        assert_eq!(exe.register.s, 0xff);
        exe.execute(); // LDA
        assert_eq!(exe.register.a, 0x00);

        println!("cycles_dt: {}", exe.cycles - cycles_beg);

        exe.execute(); // STA
        exe.execute(); // STA

        assert_eq!(exe.ppu_unit.get_ppu_register().ppuaddr, 0x0000);

        exe.execute(); // LDA
        exe.execute(); // STA
        exe.execute(); // LDA
        exe.execute(); // STA

        assert_eq!(exe.ppu_unit.get_ppu_register().ppuaddr, 0x3f00);

        exe.execute(); // LDX
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDX);
        exe.execute(); // LDY
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDY);
        assert_eq!(exe.last_exec_inst.operand, Operand::Immediate(0x10));

        exe.execute(); // LDA
        assert_eq!(exe.register.a, 0x0f);

        exe.execute(); // STA
        exe.execute(); // INX
        exe.execute(); // DEY

        exe.execute(); // BNE
        assert_eq!(exe.last_exec_inst.opcode, Opcode::BNE);

        for _ in 1..16 {
            exe.execute(); // LDA
            assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
            exe.execute(); // STA
            assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
            exe.execute(); // INX
            assert_eq!(exe.last_exec_inst.opcode, Opcode::INX);
            exe.execute(); // DEY
            assert_eq!(exe.last_exec_inst.opcode, Opcode::DEY);
            exe.execute(); // BNE
            assert_eq!(exe.last_exec_inst.opcode, Opcode::BNE);
        }

        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
        exe.execute(); // LDX
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDX);
        exe.execute(); // LDY
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDY);

        for _ in 0..0x0d {
            exe.execute(); // LDA
            assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
            exe.execute(); // STA
            assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
            exe.execute(); // INX
            assert_eq!(exe.last_exec_inst.opcode, Opcode::INX);
            exe.execute(); // DEY
            assert_eq!(exe.last_exec_inst.opcode, Opcode::DEY);
            exe.execute(); // BNE
            assert_eq!(exe.last_exec_inst.opcode, Opcode::BNE);
        }

        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);

        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
        exe.execute(); // STA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);

        println!("cycles_dt: {}", exe.cycles - cycles_beg);

        // 無限ループ
        for _ in 0..16 {
            exe.execute(); // JMP
            assert_eq!(exe.last_exec_inst.opcode, Opcode::JMP);
        }

        //println!("xxx: last_exec_inst: {:#?}", exe.last_exec_inst);
    }

    #[test]
    fn test_cpu() {
        use crate::rom::Rom;

        assert!(true);
        assert_eq!(1, 1);

        let rom = Rom::load_image("static/sample1/sample1.nes".to_string());

        let prg = rom.get_prg();
        assert_eq!(rom.get_bytes_of_prg(), prg.len());

        println!("prog len: {}", prg.len());

        let mut cur = std::io::Cursor::new(prg);

        let expect_insts = &[
            (Opcode::SEI, Operand::Implied),
            (Opcode::LDX, Operand::Immediate(255)),
            (Opcode::TXS, Operand::Implied),
            (Opcode::LDA, Operand::Immediate(0)),
            (Opcode::STA, Operand::Absolute(8192)),
            (Opcode::STA, Operand::Absolute(8193)),
            (Opcode::LDA, Operand::Immediate(63)),
            (Opcode::STA, Operand::Absolute(8198)),
            (Opcode::LDA, Operand::Immediate(0)),
            (Opcode::STA, Operand::Absolute(8198)),
            (Opcode::LDX, Operand::Immediate(0)),
            (Opcode::LDY, Operand::Immediate(16)),
            (Opcode::LDA, Operand::AbsoluteX(32849)),
            (Opcode::STA, Operand::Absolute(8199)),
            (Opcode::INX, Operand::Implied),
            (Opcode::DEY, Operand::Implied),
            (Opcode::BNE, Operand::Relative(246)),
        ];

        for i in 0..17 {
            let op = cur.read_u8().unwrap();
            let inst_spec = &INST_SPECS[op as usize];
            let pc = i as u16; // dummy for pc
            let inst = Inst::decode(&mut cur, inst_spec, pc);

            if i < expect_insts.len() {
                assert_eq!(inst.opcode, expect_insts[i].0);
                assert_eq!(inst.operand, expect_insts[i].1);
            }
        }
    }
}
