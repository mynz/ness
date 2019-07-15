#![allow(dead_code)]

mod inst_specs;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

fn u8_to_i8(u: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(u) }
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
    fn decode(cur: &mut Cursor<&[u8]>, spec: &InstSpec) -> Self {
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
    ];

    for i in 0..6 {
        println!("i: {}", i);

        let op = cur.read_u8().unwrap();
        let inst_spec = &INST_SPECS[op as usize];
        let inst = Inst::decode(&mut cur, inst_spec);

        println!("inst: {:#?}", inst);

        assert_eq!(inst.opcode, expect_insts[i].0);
        assert_eq!(inst.operand, expect_insts[i].1);
    }
}
