#![allow(dead_code)]

extern crate rustness;
use rustness::machine::Executer;
use rustness::rom::Rom;
use rustness::*;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

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

struct Rustness {
    exe: Executer,
}

impl Rustness {
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

    fn run(rom: Rom, _args: CmdArgs) {
        let win_size = Vector::new(255, 240);
        run_with("RUSTNESS", win_size, Settings::default(), || {
            Self::new_with_params(Some(rom))
        });
    }
}

impl State for Rustness {
    fn new() -> Result<Rustness> {
        Self::new()
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Q].is_down() || window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        let mut keybits: u32 = 0;

        let keys = window.keyboard();
        if keys[Key::A].is_down() {
            keybits |= 1 << PadButton::Left as u8;
        }
        if keys[Key::S].is_down() {
            keybits |= 1 << PadButton::Down as u8;
        }
        if keys[Key::W].is_down() {
            keybits |= 1 << PadButton::Up as u8;
        }
        if keys[Key::D].is_down() {
            keybits |= 1 << PadButton::Right as u8;
        }

        if keys[Key::K].is_down() {
            keybits |= 1 << PadButton::A as u8;
        }
        if keys[Key::J].is_down() {
            keybits |= 1 << PadButton::B as u8;
        }

        if keys[Key::G].is_down() {
            keybits |= 1 << PadButton::Select as u8;
        }
        if keys[Key::H].is_down() {
            keybits |= 1 << PadButton::Start as u8;
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

        if self.exe.args.debug_level > 0 {
            println!("update frame: {}", frame_count);
        }

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
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("-q, --quiet 'Set stdout silent'"))
        .arg(Arg::from_usage("-r, --rom [ROM] 'Set rom file path'"))
        .arg(Arg::from_usage(
            "-d, --debug-level=[LEVEL] 'Set debug level'",
        ));

    let matches = app.get_matches();

    let debug_level = matches
        .value_of("debug-level")
        .map_or(1, |v| v.parse().unwrap());

    let rom_path = matches.value_of("ROM").unwrap_or(
        //"static/sample1/sample1.nes"
        //"static/roms/giko005.nes"
        //"static/roms/giko008.nes"
        //"static/roms/giko009.nes"
        //"static/roms/giko010b.nes"
        //"static/roms/giko011.nes"
        //"static/roms/giko016.nes" // NG
        //"static/roms/falling.nes" // NG
        //"static/roms/nestest.nes" // OK
        "static/local/Super_mario_brothers.nes", // NG
    );

    let args = CmdArgs {
        debug_level,
        rom_path: rom_path.to_string(),
    };

    if false || args.debug_level > 0 {
        println!("rom: [{}]", rom_path);
        println!("debug-level: {}", debug_level);
    }

    let rom = Rom::load_image(rom_path);
    Rustness::run(rom, args);
}
