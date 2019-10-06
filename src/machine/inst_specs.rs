use super::*;

pub(super) const INST_SPECS: &[InstSpec] = &[
    InstSpec {
        // 0
        code: 0x0,
        opcode: Opcode::BRK,
        operand: Operand::Implied,
        size: 1,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 1
        code: 0x1,
        opcode: Opcode::ORA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 2
        code: 0x2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 3
        code: 0x3,
        opcode: Opcode::SLO,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 8,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 4
        code: 0x4,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 5
        code: 0x5,
        opcode: Opcode::ORA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 6
        code: 0x6,
        opcode: Opcode::ASL,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 7
        code: 0x7,
        opcode: Opcode::SLO,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 8
        code: 0x8,
        opcode: Opcode::PHP,
        operand: Operand::Implied,
        size: 1,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 9
        code: 0x9,
        opcode: Opcode::ORA,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 10
        code: 0xa,
        opcode: Opcode::ASL,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 11
        code: 0xb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 12
        code: 0xc,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 13
        code: 0xd,
        opcode: Opcode::ORA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 14
        code: 0xe,
        opcode: Opcode::ASL,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 15
        code: 0xf,
        opcode: Opcode::SLO,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 16
        code: 0x10,
        opcode: Opcode::BPL,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 17
        code: 0x11,
        opcode: Opcode::ORA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 18
        code: 0x12,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 19
        code: 0x13,
        opcode: Opcode::SLO,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 8,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 20
        code: 0x14,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 21
        code: 0x15,
        opcode: Opcode::ORA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 22
        code: 0x16,
        opcode: Opcode::ASL,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 23
        code: 0x17,
        opcode: Opcode::SLO,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 24
        code: 0x18,
        opcode: Opcode::CLC,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 25
        code: 0x19,
        opcode: Opcode::ORA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 26
        code: 0x1a,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 27
        code: 0x1b,
        opcode: Opcode::SLO,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 28
        code: 0x1c,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 29
        code: 0x1d,
        opcode: Opcode::ORA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 30
        code: 0x1e,
        opcode: Opcode::ASL,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 31
        code: 0x1f,
        opcode: Opcode::SLO,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 32
        code: 0x20,
        opcode: Opcode::JSR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 33
        code: 0x21,
        opcode: Opcode::AND,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 34
        code: 0x22,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 35
        code: 0x23,
        opcode: Opcode::RLA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 8,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 36
        code: 0x24,
        opcode: Opcode::BIT,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 37
        code: 0x25,
        opcode: Opcode::AND,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 38
        code: 0x26,
        opcode: Opcode::ROL,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 39
        code: 0x27,
        opcode: Opcode::RLA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 40
        code: 0x28,
        opcode: Opcode::PLP,
        operand: Operand::Implied,
        size: 1,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 41
        code: 0x29,
        opcode: Opcode::AND,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 42
        code: 0x2a,
        opcode: Opcode::ROL,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 43
        code: 0x2b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 44
        code: 0x2c,
        opcode: Opcode::BIT,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 45
        code: 0x2d,
        opcode: Opcode::AND,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 46
        code: 0x2e,
        opcode: Opcode::ROL,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 47
        code: 0x2f,
        opcode: Opcode::RLA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 48
        code: 0x30,
        opcode: Opcode::BMI,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 49
        code: 0x31,
        opcode: Opcode::AND,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 50
        code: 0x32,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 51
        code: 0x33,
        opcode: Opcode::RLA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 8,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 52
        code: 0x34,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 53
        code: 0x35,
        opcode: Opcode::AND,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 54
        code: 0x36,
        opcode: Opcode::ROL,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 55
        code: 0x37,
        opcode: Opcode::RLA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 56
        code: 0x38,
        opcode: Opcode::SEC,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 57
        code: 0x39,
        opcode: Opcode::AND,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 58
        code: 0x3a,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 59
        code: 0x3b,
        opcode: Opcode::RLA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 60
        code: 0x3c,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 61
        code: 0x3d,
        opcode: Opcode::AND,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 62
        code: 0x3e,
        opcode: Opcode::ROL,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 63
        code: 0x3f,
        opcode: Opcode::RLA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 64
        code: 0x40,
        opcode: Opcode::RTI,
        operand: Operand::Implied,
        size: 1,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 65
        code: 0x41,
        opcode: Opcode::EOR,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 66
        code: 0x42,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 67
        code: 0x43,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 68
        code: 0x44,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 69
        code: 0x45,
        opcode: Opcode::EOR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 70
        code: 0x46,
        opcode: Opcode::LSR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 71
        code: 0x47,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 72
        code: 0x48,
        opcode: Opcode::PHA,
        operand: Operand::Implied,
        size: 1,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 73
        code: 0x49,
        opcode: Opcode::EOR,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 74
        code: 0x4a,
        opcode: Opcode::LSR,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 75
        code: 0x4b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 76
        code: 0x4c,
        opcode: Opcode::JMP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 77
        code: 0x4d,
        opcode: Opcode::EOR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 78
        code: 0x4e,
        opcode: Opcode::LSR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 79
        code: 0x4f,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 80
        code: 0x50,
        opcode: Opcode::BVC,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 81
        code: 0x51,
        opcode: Opcode::EOR,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 82
        code: 0x52,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 83
        code: 0x53,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 84
        code: 0x54,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 85
        code: 0x55,
        opcode: Opcode::EOR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 86
        code: 0x56,
        opcode: Opcode::LSR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 87
        code: 0x57,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 88
        code: 0x58,
        opcode: Opcode::CLI,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 89
        code: 0x59,
        opcode: Opcode::EOR,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 90
        code: 0x5a,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 91
        code: 0x5b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 92
        code: 0x5c,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 93
        code: 0x5d,
        opcode: Opcode::EOR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 94
        code: 0x5e,
        opcode: Opcode::LSR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 95
        code: 0x5f,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 96
        code: 0x60,
        opcode: Opcode::RTS,
        operand: Operand::Implied,
        size: 1,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 97
        code: 0x61,
        opcode: Opcode::ADC,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 98
        code: 0x62,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 99
        code: 0x63,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 100
        code: 0x64,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 101
        code: 0x65,
        opcode: Opcode::ADC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 102
        code: 0x66,
        opcode: Opcode::ROR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 103
        code: 0x67,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 104
        code: 0x68,
        opcode: Opcode::PLA,
        operand: Operand::Implied,
        size: 1,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 105
        code: 0x69,
        opcode: Opcode::ADC,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 106
        code: 0x6a,
        opcode: Opcode::ROR,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 107
        code: 0x6b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 108
        code: 0x6c,
        opcode: Opcode::JMP,
        operand: Operand::Indirect(0),
        size: 3,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 109
        code: 0x6d,
        opcode: Opcode::ADC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 110
        code: 0x6e,
        opcode: Opcode::ROR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 111
        code: 0x6f,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 112
        code: 0x70,
        opcode: Opcode::BVS,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 113
        code: 0x71,
        opcode: Opcode::ADC,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 114
        code: 0x72,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 115
        code: 0x73,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 116
        code: 0x74,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 117
        code: 0x75,
        opcode: Opcode::ADC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 118
        code: 0x76,
        opcode: Opcode::ROR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 119
        code: 0x77,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 120
        code: 0x78,
        opcode: Opcode::SEI,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 121
        code: 0x79,
        opcode: Opcode::ADC,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 122
        code: 0x7a,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 123
        code: 0x7b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 124
        code: 0x7c,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 125
        code: 0x7d,
        opcode: Opcode::ADC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 126
        code: 0x7e,
        opcode: Opcode::ROR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 127
        code: 0x7f,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 128
        code: 0x80,
        opcode: Opcode::NOP,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 129
        code: 0x81,
        opcode: Opcode::STA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 130
        code: 0x82,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 131
        code: 0x83,
        opcode: Opcode::SAX,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 132
        code: 0x84,
        opcode: Opcode::STY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 133
        code: 0x85,
        opcode: Opcode::STA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 134
        code: 0x86,
        opcode: Opcode::STX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 135
        code: 0x87,
        opcode: Opcode::SAX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 136
        code: 0x88,
        opcode: Opcode::DEY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 137
        code: 0x89,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 138
        code: 0x8a,
        opcode: Opcode::TXA,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 139
        code: 0x8b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 140
        code: 0x8c,
        opcode: Opcode::STY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 141
        code: 0x8d,
        opcode: Opcode::STA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 142
        code: 0x8e,
        opcode: Opcode::STX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 143
        code: 0x8f,
        opcode: Opcode::SAX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 144
        code: 0x90,
        opcode: Opcode::BCC,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 145
        code: 0x91,
        opcode: Opcode::STA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 146
        code: 0x92,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 147
        code: 0x93,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 148
        code: 0x94,
        opcode: Opcode::STY,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 149
        code: 0x95,
        opcode: Opcode::STA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 150
        code: 0x96,
        opcode: Opcode::STX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 151
        code: 0x97,
        opcode: Opcode::SAX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 152
        code: 0x98,
        opcode: Opcode::TYA,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 153
        code: 0x99,
        opcode: Opcode::STA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 154
        code: 0x9a,
        opcode: Opcode::TXS,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 155
        code: 0x9b,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 156
        code: 0x9c,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 157
        code: 0x9d,
        opcode: Opcode::STA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 158
        code: 0x9e,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 159
        code: 0x9f,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 160
        code: 0xa0,
        opcode: Opcode::LDY,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 161
        code: 0xa1,
        opcode: Opcode::LDA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 162
        code: 0xa2,
        opcode: Opcode::LDX,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 163
        code: 0xa3,
        opcode: Opcode::LAX,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 164
        code: 0xa4,
        opcode: Opcode::LDY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 165
        code: 0xa5,
        opcode: Opcode::LDA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 166
        code: 0xa6,
        opcode: Opcode::LDX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 167
        code: 0xa7,
        opcode: Opcode::LAX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 168
        code: 0xa8,
        opcode: Opcode::TAY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 169
        code: 0xa9,
        opcode: Opcode::LDA,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 170
        code: 0xaa,
        opcode: Opcode::TAX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 171
        code: 0xab,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 172
        code: 0xac,
        opcode: Opcode::LDY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 173
        code: 0xad,
        opcode: Opcode::LDA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 174
        code: 0xae,
        opcode: Opcode::LDX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 175
        code: 0xaf,
        opcode: Opcode::LAX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 176
        code: 0xb0,
        opcode: Opcode::BCS,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 177
        code: 0xb1,
        opcode: Opcode::LDA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 178
        code: 0xb2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 179
        code: 0xb3,
        opcode: Opcode::LAX,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 180
        code: 0xb4,
        opcode: Opcode::LDY,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 181
        code: 0xb5,
        opcode: Opcode::LDA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 182
        code: 0xb6,
        opcode: Opcode::LDX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 183
        code: 0xb7,
        opcode: Opcode::LAX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 184
        code: 0xb8,
        opcode: Opcode::CLV,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 185
        code: 0xb9,
        opcode: Opcode::LDA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 186
        code: 0xba,
        opcode: Opcode::TSX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 187
        code: 0xbb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 188
        code: 0xbc,
        opcode: Opcode::LDY,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 189
        code: 0xbd,
        opcode: Opcode::LDA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 190
        code: 0xbe,
        opcode: Opcode::LDX,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 191
        code: 0xbf,
        opcode: Opcode::LAX,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 192
        code: 0xc0,
        opcode: Opcode::CPY,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 193
        code: 0xc1,
        opcode: Opcode::CMP,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 194
        code: 0xc2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 195
        code: 0xc3,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 196
        code: 0xc4,
        opcode: Opcode::CPY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 197
        code: 0xc5,
        opcode: Opcode::CMP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 198
        code: 0xc6,
        opcode: Opcode::DEC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 199
        code: 0xc7,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 200
        code: 0xc8,
        opcode: Opcode::INY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 201
        code: 0xc9,
        opcode: Opcode::CMP,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 202
        code: 0xca,
        opcode: Opcode::DEX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 203
        code: 0xcb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 204
        code: 0xcc,
        opcode: Opcode::CPY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 205
        code: 0xcd,
        opcode: Opcode::CMP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 206
        code: 0xce,
        opcode: Opcode::DEC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 207
        code: 0xcf,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 208
        code: 0xd0,
        opcode: Opcode::BNE,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 209
        code: 0xd1,
        opcode: Opcode::CMP,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 210
        code: 0xd2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 211
        code: 0xd3,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 212
        code: 0xd4,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 213
        code: 0xd5,
        opcode: Opcode::CMP,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 214
        code: 0xd6,
        opcode: Opcode::DEC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 215
        code: 0xd7,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 216
        code: 0xd8,
        opcode: Opcode::CLD,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 217
        code: 0xd9,
        opcode: Opcode::CMP,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 218
        code: 0xda,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 219
        code: 0xdb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 220
        code: 0xdc,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 221
        code: 0xdd,
        opcode: Opcode::CMP,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 222
        code: 0xde,
        opcode: Opcode::DEC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 223
        code: 0xdf,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 224
        code: 0xe0,
        opcode: Opcode::CPX,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 225
        code: 0xe1,
        opcode: Opcode::SBC,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 226
        code: 0xe2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 227
        code: 0xe3,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 228
        code: 0xe4,
        opcode: Opcode::CPX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 229
        code: 0xe5,
        opcode: Opcode::SBC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 230
        code: 0xe6,
        opcode: Opcode::INC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 231
        code: 0xe7,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 232
        code: 0xe8,
        opcode: Opcode::INX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 233
        code: 0xe9,
        opcode: Opcode::SBC,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 234
        code: 0xe7,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 235
        code: 0xeb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 236
        code: 0xec,
        opcode: Opcode::CPX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 237
        code: 0xed,
        opcode: Opcode::SBC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 238
        code: 0xee,
        opcode: Opcode::INC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 239
        code: 0xef,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 240
        code: 0xf0,
        opcode: Opcode::BEQ,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 241
        code: 0xf1,
        opcode: Opcode::SBC,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 242
        code: 0xf2,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 243
        code: 0xf3,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 244
        code: 0xf4,
        opcode: Opcode::NOP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 245
        code: 0xf5,
        opcode: Opcode::SBC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 246
        code: 0xf6,
        opcode: Opcode::INC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 247
        code: 0xf7,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 248
        code: 0xf8,
        opcode: Opcode::SED,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 249
        code: 0xf9,
        opcode: Opcode::SBC,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 250
        code: 0xfa,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 251
        code: 0xfb,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 252
        code: 0xfc,
        opcode: Opcode::NOP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
    InstSpec {
        // 253
        code: 0xfd,
        opcode: Opcode::SBC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 254
        code: 0xfe,
        opcode: Opcode::INC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 255
        code: 0xff,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Undefined,
    },
];
