use super::*;

pub(super) const INST_SPECS: &[InstSpec] = &[
    InstSpec {
        // 0
        code: 0,
        opcode: Opcode::BRK,
        operand: Operand::Implied,
        size: 1,
        cycles: 7,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 1
        code: 1,
        opcode: Opcode::ORA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 2
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 3
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 4
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 5
        code: 5,
        opcode: Opcode::ORA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 6
        code: 6,
        opcode: Opcode::ASL,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 7
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 8
        code: 8,
        opcode: Opcode::PHP,
        operand: Operand::Implied,
        size: 1,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 9
        code: 9,
        opcode: Opcode::ORA,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 10
        code: 10,
        opcode: Opcode::ASL,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 11
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 12
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 13
        code: 13,
        opcode: Opcode::ORA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 14
        code: 14,
        opcode: Opcode::ASL,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 15
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 16
        code: 16,
        opcode: Opcode::BPL,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 17
        code: 17,
        opcode: Opcode::ORA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 18
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 19
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 20
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 21
        code: 21,
        opcode: Opcode::ORA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 22
        code: 22,
        opcode: Opcode::ASL,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 23
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 24
        code: 24,
        opcode: Opcode::CLC,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 25
        code: 25,
        opcode: Opcode::ORA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 26
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 27
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 28
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 29
        code: 29,
        opcode: Opcode::ORA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 30
        code: 30,
        opcode: Opcode::ASL,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 31
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 32
        code: 32,
        opcode: Opcode::JSR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 33
        code: 33,
        opcode: Opcode::AND,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 34
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 35
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 36
        code: 36,
        opcode: Opcode::BIT,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 37
        code: 37,
        opcode: Opcode::AND,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 38
        code: 38,
        opcode: Opcode::ROL,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 39
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 40
        code: 40,
        opcode: Opcode::PLP,
        operand: Operand::Implied,
        size: 1,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 41
        code: 41,
        opcode: Opcode::AND,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 42
        code: 42,
        opcode: Opcode::ROL,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 43
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 44
        code: 44,
        opcode: Opcode::BIT,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 45
        code: 45,
        opcode: Opcode::AND,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 46
        code: 46,
        opcode: Opcode::ROL,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 47
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 48
        code: 48,
        opcode: Opcode::BMI,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 49
        code: 49,
        opcode: Opcode::AND,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 50
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 51
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 52
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 53
        code: 53,
        opcode: Opcode::AND,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 54
        code: 54,
        opcode: Opcode::ROL,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 55
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 56
        code: 56,
        opcode: Opcode::SEC,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 57
        code: 57,
        opcode: Opcode::AND,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 58
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 59
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 60
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 61
        code: 61,
        opcode: Opcode::AND,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 62
        code: 62,
        opcode: Opcode::ROL,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 63
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 64
        code: 64,
        opcode: Opcode::RTI,
        operand: Operand::Implied,
        size: 1,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 65
        code: 65,
        opcode: Opcode::EOR,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 66
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 67
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 68
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 69
        code: 69,
        opcode: Opcode::EOR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 70
        code: 70,
        opcode: Opcode::LSR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 71
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 72
        code: 72,
        opcode: Opcode::PHA,
        operand: Operand::Implied,
        size: 1,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 73
        code: 73,
        opcode: Opcode::EOR,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 74
        code: 74,
        opcode: Opcode::LSR,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 75
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 76
        code: 76,
        opcode: Opcode::JMP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 77
        code: 77,
        opcode: Opcode::EOR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 78
        code: 78,
        opcode: Opcode::LSR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 79
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 80
        code: 80,
        opcode: Opcode::BVC,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 81
        code: 81,
        opcode: Opcode::EOR,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 82
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 83
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 84
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 85
        code: 85,
        opcode: Opcode::EOR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 86
        code: 86,
        opcode: Opcode::LSR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 87
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 88
        code: 88,
        opcode: Opcode::CLI,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 89
        code: 89,
        opcode: Opcode::EOR,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 90
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 91
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 92
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 93
        code: 93,
        opcode: Opcode::EOR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 94
        code: 94,
        opcode: Opcode::LSR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 95
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 96
        code: 96,
        opcode: Opcode::RTS,
        operand: Operand::Implied,
        size: 1,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 97
        code: 97,
        opcode: Opcode::ADC,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 98
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 99
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 100
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 101
        code: 101,
        opcode: Opcode::ADC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 102
        code: 102,
        opcode: Opcode::ROR,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 103
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 104
        code: 104,
        opcode: Opcode::PLA,
        operand: Operand::Implied,
        size: 1,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 105
        code: 105,
        opcode: Opcode::ADC,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 106
        code: 106,
        opcode: Opcode::ROR,
        operand: Operand::Accumulator,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 107
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 108
        code: 108,
        opcode: Opcode::JMP,
        operand: Operand::Indirect(0),
        size: 3,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 109
        code: 109,
        opcode: Opcode::ADC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 110
        code: 110,
        opcode: Opcode::ROR,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 111
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 112
        code: 112,
        opcode: Opcode::BVS,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 113
        code: 113,
        opcode: Opcode::ADC,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 114
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 115
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 116
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 117
        code: 117,
        opcode: Opcode::ADC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 118
        code: 118,
        opcode: Opcode::ROR,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 119
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 120
        code: 120,
        opcode: Opcode::SEI,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 121
        code: 121,
        opcode: Opcode::ADC,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 122
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 123
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 124
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 125
        code: 125,
        opcode: Opcode::ADC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 126
        code: 126,
        opcode: Opcode::ROR,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 127
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 128
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 129
        code: 129,
        opcode: Opcode::STA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 130
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 131
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 132
        code: 132,
        opcode: Opcode::STY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 133
        code: 133,
        opcode: Opcode::STA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 134
        code: 134,
        opcode: Opcode::STX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 135
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 136
        code: 136,
        opcode: Opcode::DEY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 137
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 138
        code: 138,
        opcode: Opcode::TXA,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 139
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 140
        code: 140,
        opcode: Opcode::STY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 141
        code: 141,
        opcode: Opcode::STA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 142
        code: 142,
        opcode: Opcode::STX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 143
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 144
        code: 144,
        opcode: Opcode::BCC,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 145
        code: 145,
        opcode: Opcode::STA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 146
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 147
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 148
        code: 148,
        opcode: Opcode::STY,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 149
        code: 149,
        opcode: Opcode::STA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 150
        code: 150,
        opcode: Opcode::STX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 151
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 152
        code: 152,
        opcode: Opcode::TYA,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 153
        code: 153,
        opcode: Opcode::STA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 154
        code: 154,
        opcode: Opcode::TXS,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 155
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 156
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 157
        code: 157,
        opcode: Opcode::STA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 158
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 159
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 160
        code: 160,
        opcode: Opcode::LDY,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 161
        code: 161,
        opcode: Opcode::LDA,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 162
        code: 162,
        opcode: Opcode::LDX,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 163
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 164
        code: 164,
        opcode: Opcode::LDY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 165
        code: 165,
        opcode: Opcode::LDA,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 166
        code: 166,
        opcode: Opcode::LDX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 167
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 168
        code: 168,
        opcode: Opcode::TAY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 169
        code: 169,
        opcode: Opcode::LDA,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 170
        code: 170,
        opcode: Opcode::TAX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 171
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 172
        code: 172,
        opcode: Opcode::LDY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 173
        code: 173,
        opcode: Opcode::LDA,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 174
        code: 174,
        opcode: Opcode::LDX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 175
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 176
        code: 176,
        opcode: Opcode::BCS,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 177
        code: 177,
        opcode: Opcode::LDA,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 178
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 179
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 180
        code: 180,
        opcode: Opcode::LDY,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 181
        code: 181,
        opcode: Opcode::LDA,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 182
        code: 182,
        opcode: Opcode::LDX,
        operand: Operand::ZeroPageY(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 183
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 184
        code: 184,
        opcode: Opcode::CLV,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 185
        code: 185,
        opcode: Opcode::LDA,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 186
        code: 186,
        opcode: Opcode::TSX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 187
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 188
        code: 188,
        opcode: Opcode::LDY,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 189
        code: 189,
        opcode: Opcode::LDA,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 190
        code: 190,
        opcode: Opcode::LDX,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 191
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 192
        code: 192,
        opcode: Opcode::CPY,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 193
        code: 193,
        opcode: Opcode::CMP,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 194
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 195
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 196
        code: 196,
        opcode: Opcode::CPY,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 197
        code: 197,
        opcode: Opcode::CMP,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 198
        code: 198,
        opcode: Opcode::DEC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 199
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 200
        code: 200,
        opcode: Opcode::INY,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 201
        code: 201,
        opcode: Opcode::CMP,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 202
        code: 202,
        opcode: Opcode::DEX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 203
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 204
        code: 204,
        opcode: Opcode::CPY,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 205
        code: 205,
        opcode: Opcode::CMP,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 206
        code: 206,
        opcode: Opcode::DEC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 207
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 208
        code: 208,
        opcode: Opcode::BNE,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 209
        code: 209,
        opcode: Opcode::CMP,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 210
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 211
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 212
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 213
        code: 213,
        opcode: Opcode::CMP,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 214
        code: 214,
        opcode: Opcode::DEC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 215
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 216
        code: 216,
        opcode: Opcode::CLD,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 217
        code: 217,
        opcode: Opcode::CMP,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 218
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 219
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 220
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 221
        code: 221,
        opcode: Opcode::CMP,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 222
        code: 222,
        opcode: Opcode::DEC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 223
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 224
        code: 224,
        opcode: Opcode::CPX,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 225
        code: 225,
        opcode: Opcode::SBC,
        operand: Operand::IndirectX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 226
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 227
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 228
        code: 228,
        opcode: Opcode::CPX,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 229
        code: 229,
        opcode: Opcode::SBC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 3,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 230
        code: 230,
        opcode: Opcode::INC,
        operand: Operand::ZeroPage(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 231
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 232
        code: 232,
        opcode: Opcode::INX,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 233
        code: 233,
        opcode: Opcode::SBC,
        operand: Operand::Immediate(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 234
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 235
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 236
        code: 236,
        opcode: Opcode::CPX,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 237
        code: 237,
        opcode: Opcode::SBC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 238
        code: 238,
        opcode: Opcode::INC,
        operand: Operand::Absolute(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 239
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 240
        code: 240,
        opcode: Opcode::BEQ,
        operand: Operand::Relative(0),
        size: 2,
        cycles: 2,
        ext_cycles: ExtCycles::OneOrTwo,
    },
    InstSpec {
        // 241
        code: 241,
        opcode: Opcode::SBC,
        operand: Operand::IndirectY(0),
        size: 2,
        cycles: 5,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 242
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 243
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 244
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 245
        code: 245,
        opcode: Opcode::SBC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 4,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 246
        code: 246,
        opcode: Opcode::INC,
        operand: Operand::ZeroPageX(0),
        size: 2,
        cycles: 6,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 247
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 248
        code: 248,
        opcode: Opcode::SED,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 249
        code: 249,
        opcode: Opcode::SBC,
        operand: Operand::AbsoluteY(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 250
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 251
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 252
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
    InstSpec {
        // 253
        code: 253,
        opcode: Opcode::SBC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 4,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 254
        code: 254,
        opcode: Opcode::INC,
        operand: Operand::AbsoluteX(0),
        size: 3,
        cycles: 6,
        ext_cycles: ExtCycles::One,
    },
    InstSpec {
        // 255
        code: 234,
        opcode: Opcode::NOP,
        operand: Operand::Implied,
        size: 1,
        cycles: 2,
        ext_cycles: ExtCycles::Zero,
    },
];
