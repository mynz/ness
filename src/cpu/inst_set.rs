use super::*;

pub(super) const INST_SET: &[Inst] = &[
    Inst {
        // 0
        opcode: Opcode::BRK,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 7,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 1
        opcode: Opcode::ORA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 2
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 3
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 4
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 5
        opcode: Opcode::ORA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 6
        opcode: Opcode::ASL,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 7
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 8
        opcode: Opcode::PHP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 9
        opcode: Opcode::ORA,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 10
        opcode: Opcode::ASL,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 11
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 12
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 13
        opcode: Opcode::ORA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 14
        opcode: Opcode::ASL,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 15
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 16
        opcode: Opcode::BPL,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 17
        opcode: Opcode::ORA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 18
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 19
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 20
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 21
        opcode: Opcode::ORA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 22
        opcode: Opcode::ASL,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 23
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 24
        opcode: Opcode::CLC,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 25
        opcode: Opcode::ORA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 26
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 27
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 28
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 29
        opcode: Opcode::ORA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 30
        opcode: Opcode::ASL,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 31
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 32
        opcode: Opcode::JSR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 33
        opcode: Opcode::AND,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 34
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 35
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 36
        opcode: Opcode::BIT,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 37
        opcode: Opcode::AND,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 38
        opcode: Opcode::ROL,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 39
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 40
        opcode: Opcode::PLP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 41
        opcode: Opcode::AND,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 42
        opcode: Opcode::ROL,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 43
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 44
        opcode: Opcode::BIT,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 45
        opcode: Opcode::AND,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 46
        opcode: Opcode::ROL,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 47
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 48
        opcode: Opcode::BMI,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 49
        opcode: Opcode::AND,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 50
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 51
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 52
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 53
        opcode: Opcode::AND,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 54
        opcode: Opcode::ROL,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 55
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 56
        opcode: Opcode::SEC,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 57
        opcode: Opcode::AND,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 58
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 59
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 60
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 61
        opcode: Opcode::AND,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 62
        opcode: Opcode::ROL,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 63
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 64
        opcode: Opcode::RTI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 65
        opcode: Opcode::EOR,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 66
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 67
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 68
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 69
        opcode: Opcode::EOR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 70
        opcode: Opcode::LSR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 71
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 72
        opcode: Opcode::PHA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 73
        opcode: Opcode::EOR,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 74
        opcode: Opcode::LSR,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 75
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 76
        opcode: Opcode::JMP,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 77
        opcode: Opcode::EOR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 78
        opcode: Opcode::LSR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 79
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 80
        opcode: Opcode::BVC,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 81
        opcode: Opcode::EOR,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 82
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 83
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 84
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 85
        opcode: Opcode::EOR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 86
        opcode: Opcode::LSR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 87
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 88
        opcode: Opcode::CLI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 89
        opcode: Opcode::EOR,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 90
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 91
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 92
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 93
        opcode: Opcode::EOR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 94
        opcode: Opcode::LSR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 95
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 96
        opcode: Opcode::RTS,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 97
        opcode: Opcode::ADC,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 98
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 99
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 100
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 101
        opcode: Opcode::ADC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 102
        opcode: Opcode::ROR,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 103
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 104
        opcode: Opcode::PLA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 105
        opcode: Opcode::ADC,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 106
        opcode: Opcode::ROR,
        addr_mode: AddrMode::Accumulator,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 107
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 108
        opcode: Opcode::JMP,
        addr_mode: AddrMode::Indirect(0),
        size: 3,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 109
        opcode: Opcode::ADC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 110
        opcode: Opcode::ROR,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 111
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 112
        opcode: Opcode::BVS,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 113
        opcode: Opcode::ADC,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 114
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 115
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 116
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 117
        opcode: Opcode::ADC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 118
        opcode: Opcode::ROR,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 119
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 120
        opcode: Opcode::SEI,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 121
        opcode: Opcode::ADC,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 122
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 123
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 124
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 125
        opcode: Opcode::ADC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 126
        opcode: Opcode::ROR,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 127
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 128
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 129
        opcode: Opcode::STA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 130
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 131
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 132
        opcode: Opcode::STY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 133
        opcode: Opcode::STA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 134
        opcode: Opcode::STX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 135
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 136
        opcode: Opcode::DEY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 137
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 138
        opcode: Opcode::TXA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 139
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 140
        opcode: Opcode::STY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 141
        opcode: Opcode::STA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 142
        opcode: Opcode::STX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 143
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 144
        opcode: Opcode::BCC,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 145
        opcode: Opcode::STA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 146
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 147
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 148
        opcode: Opcode::STY,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 149
        opcode: Opcode::STA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 150
        opcode: Opcode::STX,
        addr_mode: AddrMode::ZeroPageY(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 151
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 152
        opcode: Opcode::TYA,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 153
        opcode: Opcode::STA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 154
        opcode: Opcode::TXS,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 155
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 156
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 157
        opcode: Opcode::STA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 158
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 159
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 160
        opcode: Opcode::LDY,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 161
        opcode: Opcode::LDA,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 162
        opcode: Opcode::LDX,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 163
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 164
        opcode: Opcode::LDY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 165
        opcode: Opcode::LDA,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 166
        opcode: Opcode::LDX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 167
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 168
        opcode: Opcode::TAY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 169
        opcode: Opcode::LDA,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 170
        opcode: Opcode::TAX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 171
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 172
        opcode: Opcode::LDY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 173
        opcode: Opcode::LDA,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 174
        opcode: Opcode::LDX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 175
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 176
        opcode: Opcode::BCS,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 177
        opcode: Opcode::LDA,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 178
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 179
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 180
        opcode: Opcode::LDY,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 181
        opcode: Opcode::LDA,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 182
        opcode: Opcode::LDX,
        addr_mode: AddrMode::ZeroPageY(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 183
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 184
        opcode: Opcode::CLV,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 185
        opcode: Opcode::LDA,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 186
        opcode: Opcode::TSX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 187
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 188
        opcode: Opcode::LDY,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 189
        opcode: Opcode::LDA,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 190
        opcode: Opcode::LDX,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 191
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 192
        opcode: Opcode::CPY,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 193
        opcode: Opcode::CMP,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 194
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 195
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 196
        opcode: Opcode::CPY,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 197
        opcode: Opcode::CMP,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 198
        opcode: Opcode::DEC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 199
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 200
        opcode: Opcode::INY,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 201
        opcode: Opcode::CMP,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 202
        opcode: Opcode::DEX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 203
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 204
        opcode: Opcode::CPY,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 205
        opcode: Opcode::CMP,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 206
        opcode: Opcode::DEC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 207
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 208
        opcode: Opcode::BNE,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 209
        opcode: Opcode::CMP,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 210
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 211
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 212
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 213
        opcode: Opcode::CMP,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 214
        opcode: Opcode::DEC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 215
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 216
        opcode: Opcode::CLD,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 217
        opcode: Opcode::CMP,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 218
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 219
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 220
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 221
        opcode: Opcode::CMP,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 222
        opcode: Opcode::DEC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 223
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 224
        opcode: Opcode::CPX,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 225
        opcode: Opcode::SBC,
        addr_mode: AddrMode::IndirectX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 226
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 227
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 228
        opcode: Opcode::CPX,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 229
        opcode: Opcode::SBC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 3,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 230
        opcode: Opcode::INC,
        addr_mode: AddrMode::ZeroPage(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 231
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 232
        opcode: Opcode::INX,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 233
        opcode: Opcode::SBC,
        addr_mode: AddrMode::Immediate(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 234
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 235
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 236
        opcode: Opcode::CPX,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 237
        opcode: Opcode::SBC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 238
        opcode: Opcode::INC,
        addr_mode: AddrMode::Absolute(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 239
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 240
        opcode: Opcode::BEQ,
        addr_mode: AddrMode::Relative(0),
        size: 2,
        cycle: 2,
        ext_cycle: ExtCycle::OneOrTwo,
    },
    Inst {
        // 241
        opcode: Opcode::SBC,
        addr_mode: AddrMode::IndirectY(0),
        size: 2,
        cycle: 5,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 242
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 243
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 244
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 245
        opcode: Opcode::SBC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 4,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 246
        opcode: Opcode::INC,
        addr_mode: AddrMode::ZeroPageX(0),
        size: 2,
        cycle: 6,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 247
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 248
        opcode: Opcode::SED,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 249
        opcode: Opcode::SBC,
        addr_mode: AddrMode::AbsoluteY(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 250
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 251
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 252
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
    Inst {
        // 253
        opcode: Opcode::SBC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 4,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 254
        opcode: Opcode::INC,
        addr_mode: AddrMode::AbsoluteX(0),
        size: 3,
        cycle: 6,
        ext_cycle: ExtCycle::One,
    },
    Inst {
        // 255
        opcode: Opcode::NOP,
        addr_mode: AddrMode::Implied,
        size: 1,
        cycle: 2,
        ext_cycle: ExtCycle::Zero,
    },
];


