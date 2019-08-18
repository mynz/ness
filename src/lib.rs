pub mod cpu;
pub mod ppu;
pub mod rom;
pub mod color_palette;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(u32, u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);
