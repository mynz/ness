extern crate rustness;
use rustness::cpu::*;
use rustness::rom::*;

#[test]
fn test_integration_sample() {
    assert_eq!(1 + 1, 2);

    use rustness::ppu::PpuRegister;

    let r = PpuRegister::default();
    assert_eq!(r.ctrl, PpuRegister::default().ctrl);
}

#[test]
fn test_render_bg() {
    assert!(true);

    let mut exe = Executer::new();

    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    exe.set_rom(rom);
    exe.hard_reset();

    for _ in 0..100 {
        exe.execute();
    }
}
