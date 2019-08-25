// main.rs

#![allow(dead_code)]

//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use std::io::Cursor;
//use std::path::Path;

extern crate rustness;
use rustness::rom::Rom;

//use rustness::color_palette::COLOR_PALETTE;
//use rustness::RGB;

extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    input::Key,
    lifecycle::{run_with, Settings, State, Window},
    //Future,
    Result,
};

extern crate image;

struct App {
    rom: Option<Rom>,
}

impl App {
    fn new() -> Result<Self> {
        // ダミー
        Self::new_with_params(None)
    }

    fn new_with_params(rom: Option<Rom>) -> Result<Self> {
        Ok(Self { rom })
    }

    fn run(rom: Rom) {
        let win_size = Vector::new(255, 240);
        run_with("RUSTNESS", win_size, Settings::default(), || {
            Self::new_with_params(Some(rom))
        });
    }
}

impl State for App {
    fn new() -> Result<App> {
        App::new()
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Q].is_down() || window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        /*
        self.pad_state = PadState::default();
        if window.keyboard()[Key::K].is_down() {
            self.pad_state.key_ud = -1;
        }
        if window.keyboard()[Key::J].is_down() {
            self.pad_state.key_ud = 1;
        }
        if window.keyboard()[Key::H].is_down() {
            self.pad_state.key_lr = -1;
        }
        if window.keyboard()[Key::L].is_down() {
            self.pad_state.key_lr = 1;
        }
         */

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::GREEN)?;

        // 本来 update() で呼び出すべきかもしれないが、ここに書かないとキーイベントなどがうまくとれない.

        Ok(())
    }
}

fn main() {
    let rom = Rom::load_image("static/sample1/sample1.nes");
    //let rom = Rom::load_image("static/roms/giko005.nes");
    App::run(rom);
}
