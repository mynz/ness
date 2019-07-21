#![allow(dead_code)]

mod inst_specs;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

use self::inst_specs::INST_SPECS;
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
enum ExtCycle {
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
    cycle: u8,
    ext_cycle: ExtCycle,
}

#[test]
fn test_inst() {
    use self::inst_specs::INST_SPECS;

    for i in 0..256 {
        let inst_spec = &INST_SPECS[i];
        assert!(inst_spec.size > 0);
    }

    assert!(INST_SPECS[0].opcode == Opcode::BRK);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Inst {
    code: u8,
    opcode: Opcode,
    operand: Operand,
}

impl Inst {
    fn decode<T: std::io::Read>(cur: &mut T, spec: &InstSpec) -> Self {
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
            code: spec.code,
            opcode: spec.opcode,
            operand,
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
    rom: Box<Rom>,
}

impl Executer {
    fn new() -> Self {
        Executer::default()
    }

    fn set_rom(&mut self, rom: Box<Rom>) {
        self.rom = rom;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        let word = self.read_word(addr);
        (word & 0x00ff) as u8
    }

    fn read_word(&self, addr: u16) -> u16 {
        if addr <= 0x07ff {
            let mut cur = Cursor::new(&self.wram);
            cur.set_position(addr as u64);
            return cur.read_u16::<LittleEndian>().unwrap();
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

    fn hard_reset(&mut self) {
        self.register = Register::default();
        self.register.pc = self.read_word(0xfffc);
    }

    fn fetch_inst(&mut self) -> Inst {
        let op = self.read_byte(self.register.pc);
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
        Inst::decode(&mut bytes.as_ref(), inst_spec)
    }

    fn execute_inst(&mut self, inst: &Inst) {
        match inst.opcode {
            Opcode::SEI => {
                self.register.p.interrupt = true;
            }
            Opcode::LDX => {
                let v = match inst.operand {
                    Operand::Immediate(v) => v,
                    _ => panic!("yet to be implemented"),
                };
                self.register.x = v;
            }
            _ => {
                panic!("yet to not impl: {:#?}", inst);
            }
        }
    }

    fn execute(&mut self) {
        // TODO
        let inst = self.fetch_inst();

        println!("xxx: inst {:#?}: ", inst);

        self.execute_inst(&inst);
    }
}

#[test]
fn test_executer() {
    let mut exe = Executer::new();
    assert_eq!(0, exe.register.a);

    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    exe.set_rom(rom);

    exe.hard_reset();
    assert_eq!(exe.register.pc, 0x8000);
    exe.execute();
    assert_eq!(exe.register.x, 0x00);
    exe.execute();
    assert_eq!(exe.register.x, 0xff);

    //exe.execute();
    //exe.execute();
    //exe.execute();
    //exe.execute();
    //exe.execute();
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

    use self::inst_specs::INST_SPECS;

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
        //println!("i: {}", i);

        let op = cur.read_u8().unwrap();
        let inst_spec = &INST_SPECS[op as usize];
        let inst = Inst::decode(&mut cur, inst_spec);

        //println!("inst: {:#?}", inst);

        if i < expect_insts.len() {
            assert_eq!(inst.opcode, expect_insts[i].0);
            assert_eq!(inst.operand, expect_insts[i].1);
        }
    }
}
