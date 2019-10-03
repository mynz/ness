#![cfg(test)]

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
fn test_giko005() {
    let mut exe = Executer::new();
    assert_eq!(0, exe.register.a);

    let rom = Rom::load_image("static/roms/giko005.nes");
    exe.set_rom(rom);

    exe.hard_reset();

    let mut loop_count = 0;
    loop {
        exe.execute(); // LDA
        assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);

        exe.execute(); // BPL
        assert_eq!(exe.last_exec_inst.opcode, Opcode::BPL);

        //println!("pc: {}", exe.register.pc);
        if exe.register.pc != 32768 {
            break;
        }
        loop_count += 1;
    }
    println!("break loop: loop_count: {}", loop_count);

    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);

    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDX);

    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::STA);

    // 最後に JMP でループするまで回す
    loop {
        exe.execute();
        if exe.last_exec_inst.pc == 32834 {
            assert_eq!(exe.last_exec_inst.opcode, Opcode::JMP);
            break;
        }
        //println!("pc: {:#?}", exe.last_exec_inst);
    }

    // もう一度､ JMP を確認する
    exe.execute();
    assert_eq!(exe.last_exec_inst.opcode, Opcode::JMP);
}

#[test]
fn test_giko008() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/roms/giko008.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    let mut _inst_count = 0;
    loop {
        exe.execute(); // LDA
        if exe.get_frame_count() == 2 {
            break;
        }
        _inst_count += 1;
    }
}

#[test]
fn test_giko009() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/roms/giko009.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    let mut _inst_count = 0;
    loop {
        exe.execute(); // LDA
        if exe.get_frame_count() == 2 {
            break;
        }
        _inst_count += 1;
    }
}

#[test]
fn test_giko010b() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/roms/giko010b.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    let mut _inst_count = 0;
    loop {
        exe.execute(); // LDA
        if exe.get_frame_count() == 2 {
            break;
        }
        _inst_count += 1;
    }
}

#[test]
fn test_giko016() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/roms/giko016.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    let mut _inst_count = 0;
    loop {
        exe.execute();
        if exe.get_frame_count() == 5 {
            break;
        }
        _inst_count += 1;
    }
}

#[test]
#[ignore] // TODO: そのうち外してテストする
fn test_falling() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/roms/falling.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    let mut _inst_count = 0;
    loop {
        exe.execute();
        if exe.get_frame_count() == 5 {
            break;
        }
        _inst_count += 1;
    }
}

#[test]
fn test_executer_00() {
    let mut exe = Executer::new();
    assert_eq!(0, exe.register.a);

    let rom = Rom::load_image("static/sample1/sample1.nes");
    exe.set_rom(rom);

    let cycles_beg = exe.cycles;

    assert_eq!(exe.register.pc, 0x8000);
    exe.execute();
    assert_eq!(exe.register.x, 0x00);
    exe.execute();
    assert_eq!(exe.register.x, 0xff);
    assert_eq!(exe.register.s, 0xfd);
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

    let rom = Rom::load_image("static/sample1/sample1.nes");

    let prg = rom.get_prg();
    assert_eq!(rom.get_bytes_of_prg(), prg.len());

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

    for (i, item) in expect_insts.into_iter().enumerate() {
        let op = cur.read_u8().unwrap();
        let inst_spec = &INST_SPECS[op as usize];
        let pc = i as u16; // dummy for pc
        let inst = Inst::decode(&mut cur, inst_spec, pc);

        assert_eq!(inst.opcode, item.0);
        assert_eq!(inst.operand, item.1);
    }
}

#[test]
fn test_render_bg() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/sample1/sample1.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    assert_eq!(exe.ppu_unit.get_frame_count(), 0);

    loop {
        exe.execute();
        if exe.ppu_unit.get_frame_count() == 1 {
            break;
        }
    }
    assert_eq!(exe.last_exec_inst.opcode, Opcode::JMP);

    let cycles = exe.cycles;
    exe.ppu_unit
        .save_as_png(format!("screenshot/ss_test_render_bg_{:>04}.png", cycles));
}

#[test]
fn test_render_sprite() {
    let mut exe = Executer::new();

    exe.set_rom(Rom::load_image("static/roms/giko005.nes"));
    exe.hard_reset();

    assert_eq!(exe.ppu_unit.get_frame_count(), 0);

    loop {
        exe.execute();
        if exe.ppu_unit.get_frame_count() == 2 {
            break;
        }
    }
    assert_eq!(exe.last_exec_inst.opcode, Opcode::JMP);
    exe.ppu_unit
        .save_as_png("screenshot/ss_test_render_sprite.png");
}

#[test]
fn test_nestest() {
    let mut exe = Executer::new();

    exe.set_rom(Rom::load_image("static/roms/nestest.nes"));

    let mut cnt = 0;

    loop {
        exe.execute();
        cnt += 1;
        if exe.last_exec_inst.pc == 0xdf90 {
            break;
        }
    }

    assert_eq!(cnt, 3385);
    assert_eq!(exe.last_exec_inst.opcode, Opcode::LDA);
    assert_eq!(exe.register.a, 0x12);
}
