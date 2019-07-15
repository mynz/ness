#![allow(dead_code)]

mod inst_set;

use std::io::Cursor;
//use byteorder::ReadBytesExt;
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

// TODO: rename to Operand
#[derive(Debug)]
enum AddrMode {
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
    addr_mode: AddrMode,
    size: u8,
    cycle: u8,
    ext_cycle: ExtCycle,
}

#[test]
fn test_inst() {
    use self::inst_set::INST_SET;

    for i in 0..256 {
        let inst_spec = &INST_SET[i];
        assert!(inst_spec.size > 0);
    }

    assert!(INST_SET[0].opcode == Opcode::BRK);
}

#[derive(Debug)]
struct Inst {
    code: u8,
    opcode: Opcode,
    operand: AddrMode,
}

impl Inst {
    fn parse_operand(cur: &mut Cursor<&[u8]>, spec: &InstSpec) -> Self {
        //let n = (spec.size - 1) as usize;

        let operand = match spec.addr_mode {
            AddrMode::Implied => AddrMode::Implied,
            AddrMode::Accumulator => AddrMode::Accumulator,
            AddrMode::Immediate(_u8) => AddrMode::Immediate(cur.read_u8().unwrap()),
            AddrMode::ZeroPage(_u8) => AddrMode::ZeroPage(cur.read_u8().unwrap()),
            AddrMode::ZeroPageX(_u8) => AddrMode::ZeroPageX(cur.read_u8().unwrap()),

            AddrMode::ZeroPageY(_u8) => AddrMode::ZeroPageY(cur.read_u8().unwrap()),

            AddrMode::Absolute(_u16) => AddrMode::Absolute(cur.read_u16::<LittleEndian>().unwrap()),

            AddrMode::AbsoluteX(_u16) => {
                AddrMode::AbsoluteX(cur.read_u16::<LittleEndian>().unwrap())
            }
            AddrMode::AbsoluteY(_u16) => {
                AddrMode::AbsoluteY(cur.read_u16::<LittleEndian>().unwrap())
            }

            AddrMode::Relative(_u8) => AddrMode::Relative(cur.read_u8().unwrap()),
            AddrMode::Indirect(_u16) => AddrMode::Indirect(cur.read_u16::<LittleEndian>().unwrap()),
            AddrMode::IndirectX(_u8) => AddrMode::IndirectX(cur.read_u8().unwrap()),
            AddrMode::IndirectY(_u8) => AddrMode::IndirectY(cur.read_u8().unwrap()),
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
    //use self::inst_set::INST_SET;
    use crate::rom::Rom;

    assert!(true);
    assert_eq!(1, 1);

    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());

    let prg = rom.get_prg();
    assert_eq!(rom.get_bytes_of_prg(), prg.len());

    println!("prog len: {}", prg.len());

    let mut cur = std::io::Cursor::new(prg);

    //use std::io::Read;
    use self::inst_set::INST_SET;

    let expect_ops = &[
        Opcode::SEI,
        Opcode::LDX,
        Opcode::TXS,
        Opcode::LDA,
        Opcode::STA,
        Opcode::STA,
    ];

    for i in 0..6 {
        println!("i: {}", i);

        let op = cur.read_u8().unwrap();
        println!("op: {:#?}", op);

        let inst_spec = &INST_SET[op as usize];
        println!("inst_spec: {:#?}", inst_spec);

        let inst = Inst::parse_operand(&mut cur, inst_spec);

        println!("inst: {:#?}", inst);

        //let nrest = inst_spec.size - 1;
        //match nrest {
        //0 => {}
        //1 => {
        //cur.read_u8().unwrap();
        //}
        //2 => {
        //cur.read_u8().unwrap();
        //cur.read_u8().unwrap();
        //}
        //x => {
        //panic!("unexpected inst_spec: {}", x);
        //}
        //}

        assert_eq!(inst_spec.opcode, expect_ops[i]);
    }
}
