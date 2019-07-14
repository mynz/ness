use super::*;

const INST_SET: &[Inst] = &[
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
];


