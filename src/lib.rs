pub mod color_palette;
pub mod frame_buffer;
pub mod machine;
pub mod ppu;
pub mod rom;

type Cycle = u32;

pub struct DebugOptions {
    pub debug_level: u32,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self { debug_level: 0 }
    }
}

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
