pub mod color_palette;
pub mod cpu;
pub mod ppu;
pub mod rom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(u32, u32);
