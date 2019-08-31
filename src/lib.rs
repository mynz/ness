pub mod color_palette;
pub mod frame_buffer;
pub mod machine;
pub mod ppu;
pub mod rom;

pub enum PadButton {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(u32, u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);
