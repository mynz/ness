use super::*;

pub(super) const INST_SET: &[InstSpec] = &[
    InstSpec {
        // 0
        code: 0,
        opcode: Opcode::BRK,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 7,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 1
        code: 1,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 2
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 3
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 4
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 5
        code: 5,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 6
        code: 6,
        opcode: Opcode::ASL,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 7
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 8
        code: 8,
        opcode: Opcode::PHP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 9
        code: 9,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 10
        code: 10,
        opcode: Opcode::ASL,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 11
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 12
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 13
        code: 13,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 14
        code: 14,
        opcode: Opcode::ASL,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 15
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 16
        code: 16,
        opcode: Opcode::BPL,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 17
        code: 17,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 18
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 19
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 20
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 21
        code: 21,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 22
        code: 22,
        opcode: Opcode::ASL,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 23
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 24
        code: 24,
        opcode: Opcode::CLC,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 25
        code: 25,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 26
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 27
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 28
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 29
        code: 29,
        opcode: Opcode::ORA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 30
        code: 30,
        opcode: Opcode::ASL,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 31
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 32
        code: 32,
        opcode: Opcode::JSR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 33
        code: 33,
        opcode: Opcode::AND,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 34
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 35
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 36
        code: 36,
        opcode: Opcode::BIT,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 37
        code: 37,
        opcode: Opcode::AND,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 38
        code: 38,
        opcode: Opcode::ROL,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 39
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 40
        code: 40,
        opcode: Opcode::PLP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 41
        code: 41,
        opcode: Opcode::AND,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 42
        code: 42,
        opcode: Opcode::ROL,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 43
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 44
        code: 44,
        opcode: Opcode::BIT,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 45
        code: 45,
        opcode: Opcode::AND,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 46
        code: 46,
        opcode: Opcode::ROL,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 47
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 48
        code: 48,
        opcode: Opcode::BMI,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 49
        code: 49,
        opcode: Opcode::AND,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 50
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 51
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 52
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 53
        code: 53,
        opcode: Opcode::AND,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 54
        code: 54,
        opcode: Opcode::ROL,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 55
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 56
        code: 56,
        opcode: Opcode::SEC,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 57
        code: 57,
        opcode: Opcode::AND,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 58
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 59
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 60
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 61
        code: 61,
        opcode: Opcode::AND,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 62
        code: 62,
        opcode: Opcode::ROL,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 63
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 64
        code: 64,
        opcode: Opcode::RTI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 65
        code: 65,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 66
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 67
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 68
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 69
        code: 69,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 70
        code: 70,
        opcode: Opcode::LSR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 71
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 72
        code: 72,
        opcode: Opcode::PHA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 73
        code: 73,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 74
        code: 74,
        opcode: Opcode::LSR,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 75
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 76
        code: 76,
        opcode: Opcode::JMP,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 77
        code: 77,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 78
        code: 78,
        opcode: Opcode::LSR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 79
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 80
        code: 80,
        opcode: Opcode::BVC,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 81
        code: 81,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 82
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 83
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 84
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 85
        code: 85,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 86
        code: 86,
        opcode: Opcode::LSR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 87
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 88
        code: 88,
        opcode: Opcode::CLI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 89
        code: 89,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 90
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 91
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 92
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 93
        code: 93,
        opcode: Opcode::EOR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 94
        code: 94,
        opcode: Opcode::LSR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 95
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 96
        code: 96,
        opcode: Opcode::RTS,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 97
        code: 97,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 98
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 99
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 100
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 101
        code: 101,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 102
        code: 102,
        opcode: Opcode::ROR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 103
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 104
        code: 104,
        opcode: Opcode::PLA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 105
        code: 105,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 106
        code: 106,
        opcode: Opcode::ROR,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 107
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 108
        code: 108,
        opcode: Opcode::JMP,
        addr_mode: AddrMode::Indirect(0),
        size: 3,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 109
        code: 109,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 110
        code: 110,
        opcode: Opcode::ROR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 111
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 112
        code: 112,
        opcode: Opcode::BVS,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 113
        code: 113,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 114
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 115
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 116
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 117
        code: 117,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 118
        code: 118,
        opcode: Opcode::ROR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 119
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 120
        code: 120,
        opcode: Opcode::SEI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 121
        code: 121,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 122
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 123
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 124
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 125
        code: 125,
        opcode: Opcode::ADC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 126
        code: 126,
        opcode: Opcode::ROR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 127
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 128
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 129
        code: 129,
        opcode: Opcode::STA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 130
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 131
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 132
        code: 132,
        opcode: Opcode::STY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 133
        code: 133,
        opcode: Opcode::STA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 134
        code: 134,
        opcode: Opcode::STX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 135
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 136
        code: 136,
        opcode: Opcode::DEY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 137
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 138
        code: 138,
        opcode: Opcode::TXA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 139
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 140
        code: 140,
        opcode: Opcode::STY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 141
        code: 141,
        opcode: Opcode::STA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 142
        code: 142,
        opcode: Opcode::STX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 143
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 144
        code: 144,
        opcode: Opcode::BCC,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 145
        code: 145,
        opcode: Opcode::STA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 146
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 147
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 148
        code: 148,
        opcode: Opcode::STY,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 149
        code: 149,
        opcode: Opcode::STA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 150
        code: 150,
        opcode: Opcode::STX,
        addr_mode: AddrMode::ZeroPageY(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 151
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 152
        code: 152,
        opcode: Opcode::TYA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 153
        code: 153,
        opcode: Opcode::STA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 154
        code: 154,
        opcode: Opcode::TXS,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 155
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 156
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 157
        code: 157,
        opcode: Opcode::STA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 158
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 159
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 160
        code: 160,
        opcode: Opcode::LDY,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 161
        code: 161,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 162
        code: 162,
        opcode: Opcode::LDX,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 163
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 164
        code: 164,
        opcode: Opcode::LDY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 165
        code: 165,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 166
        code: 166,
        opcode: Opcode::LDX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 167
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 168
        code: 168,
        opcode: Opcode::TAY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 169
        code: 169,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 170
        code: 170,
        opcode: Opcode::TAX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 171
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 172
        code: 172,
        opcode: Opcode::LDY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 173
        code: 173,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 174
        code: 174,
        opcode: Opcode::LDX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 175
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 176
        code: 176,
        opcode: Opcode::BCS,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 177
        code: 177,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 178
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 179
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 180
        code: 180,
        opcode: Opcode::LDY,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 181
        code: 181,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 182
        code: 182,
        opcode: Opcode::LDX,
        addr_mode: AddrMode::ZeroPageY(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 183
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 184
        code: 184,
        opcode: Opcode::CLV,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 185
        code: 185,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 186
        code: 186,
        opcode: Opcode::TSX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 187
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 188
        code: 188,
        opcode: Opcode::LDY,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 189
        code: 189,
        opcode: Opcode::LDA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 190
        code: 190,
        opcode: Opcode::LDX,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 191
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 192
        code: 192,
        opcode: Opcode::CPY,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 193
        code: 193,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 194
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 195
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 196
        code: 196,
        opcode: Opcode::CPY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 197
        code: 197,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 198
        code: 198,
        opcode: Opcode::DEC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 199
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 200
        code: 200,
        opcode: Opcode::INY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 201
        code: 201,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 202
        code: 202,
        opcode: Opcode::DEX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 203
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 204
        code: 204,
        opcode: Opcode::CPY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 205
        code: 205,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 206
        code: 206,
        opcode: Opcode::DEC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 207
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 208
        code: 208,
        opcode: Opcode::BNE,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 209
        code: 209,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 210
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 211
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 212
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 213
        code: 213,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 214
        code: 214,
        opcode: Opcode::DEC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 215
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 216
        code: 216,
        opcode: Opcode::CLD,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 217
        code: 217,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 218
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 219
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 220
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 221
        code: 221,
        opcode: Opcode::CMP,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 222
        code: 222,
        opcode: Opcode::DEC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 223
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 224
        code: 224,
        opcode: Opcode::CPX,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 225
        code: 225,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 226
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 227
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 228
        code: 228,
        opcode: Opcode::CPX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 229
        code: 229,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 230
        code: 230,
        opcode: Opcode::INC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 231
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 232
        code: 232,
        opcode: Opcode::INX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 233
        code: 233,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 234
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 235
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 236
        code: 236,
        opcode: Opcode::CPX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 237
        code: 237,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 238
        code: 238,
        opcode: Opcode::INC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 239
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 240
        code: 240,
        opcode: Opcode::BEQ,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    InstSpec {
        // 241
        code: 241,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 242
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 243
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 244
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 245
        code: 245,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 246
        code: 246,
        opcode: Opcode::INC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 247
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 248
        code: 248,
        opcode: Opcode::SED,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 249
        code: 249,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 250
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 251
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 252
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    InstSpec {
        // 253
        code: 253,
        opcode: Opcode::SBC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 254
        code: 254,
        opcode: Opcode::INC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    InstSpec {
        // 255
        code: 234,
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
];
