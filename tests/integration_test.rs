extern crate rustness;
use rustness::machine::*;
use rustness::rom::*;

#[test]
fn test_exe_sample1() {
    let mut exe = Executer::new();

    let rom = Rom::load_image("static/sample1/sample1.nes");
    exe.set_rom(rom);
    exe.hard_reset();

    for _ in 0..64 {
        exe.execute();
    }
}
