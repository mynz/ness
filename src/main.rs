#![allow(dead_code)]

extern crate rustness;
use rustness::machine::Executer;
use rustness::rom::Rom;

extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    graphics::Background::Img,
    graphics::{Color, Image, PixelFormat},
    input::Key,
    lifecycle::{run_with, Settings, State, Window},
    //Future,
    Result,
};

extern crate image;

struct App {
    exe: Executer,
}

impl App {
    // ダミー
    fn new() -> Result<Self> {
        Self::new_with_params(None)
    }

    fn new_with_params(rom: Option<Rom>) -> Result<Self> {
        let mut exe = Executer::new();

        if let Some(rom) = rom {
            exe.set_rom(rom);
            exe.hard_reset();
        }

        Ok(Self { exe })
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
        use rustness::PadButton;

        if window.keyboard()[Key::Q].is_down() || window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        let mut keybits: u32 = 0;

        let keys = window.keyboard();
        if keys[Key::H].is_down() {
            keybits |= 1 << PadButton::Left as u8;
        }
        if keys[Key::J].is_down() {
            keybits |= 1 << PadButton::Down as u8;
        }
        if keys[Key::K].is_down() {
            keybits |= 1 << PadButton::Up as u8;
        }
        if keys[Key::L].is_down() {
            keybits |= 1 << PadButton::Right as u8;
        }

        // TODO: joypad1 の入力も必要
        self.exe.set_joypad_keybits(0, keybits);

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
        // 本来 update() で呼び出すべきかもしれないが、
        // ここに書かないとキーイベントなどがうまくとれない.
        let frame_count = self.exe.get_frame_count();
        loop {
            self.exe.execute();
            if self.exe.get_frame_count() > frame_count {
                break;
            }
        }

        println!("update frame: {}", frame_count);

        window.clear(Color::GREEN)?;
        {
            let fb = self.exe.get_frame_buffer();
            let img = Image::from_raw(&fb.buf, fb.w, fb.h, PixelFormat::RGB).unwrap();
            window.draw(&img.area(), Img(&img));
        }

        Ok(())
    }
}

fn main() {
    //let rom = Rom::load_image("static/sample1/sample1.nes");
    //let rom = Rom::load_image("static/roms/giko005.nes");
    //let rom = Rom::load_image("static/roms/giko008.nes");
    //let rom = Rom::load_image("static/roms/giko009.nes");
    //let rom = Rom::load_image("static/roms/giko010b.nes");
    //let rom = Rom::load_image("static/roms/giko011.nes");
    //let rom = Rom::load_image("static/roms/giko016.nes"); // NG
    //let rom = Rom::load_image("static/roms/nestest.nes"); // NG
    let rom = Rom::load_image("static/roms/falling.nes"); // NG
    App::run(rom);
}
