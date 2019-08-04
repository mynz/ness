pub mod color_palette;
pub mod cpu;
pub mod ppu;
pub mod rom;

mod tests {

#[test]
fn test_render_bg() {
    use crate::cpu::*;
    use crate::rom::*;

    let mut exe = Executer::new();

    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    exe.set_rom(rom);
    exe.hard_reset();

    for _ in 0..64 {
        exe.execute();
    }
}

}
