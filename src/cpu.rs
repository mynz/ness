#![allow(dead_code)]

mod inst_set;


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

#[derive(PartialEq)]
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

enum ExtCycle {
    Zero,
    One,
    OneOrTwo,
}

struct Inst {
    opcode: Opcode,
    addr_mode: AddrMode,
    size: u8,
    cycle: u8,
    ext_cycle: ExtCycle,
}

#[test]
fn test_cpu() {
    use crate::rom::Rom;
    use self::inst_set::INST_SET;

    assert!(true);
    assert_eq!(1, 1);

    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());

    let prg = rom.get_prg();
    assert_eq!(rom.get_bytes_of_prg(), prg.len());

    println!("prog len: {}", prg.len());

    for i in 0..256 {
        let inst = &INST_SET[i];
        assert!(inst.size > 0);
    }

    assert!(INST_SET[0].opcode == Opcode::BRK);
}
