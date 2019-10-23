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
    graphics::{Image, PixelFormat},
    input::{ButtonState, Key},
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
        let args = CmdArgs::default();
        Self::new_with_params(None, args)
    }

    fn new_with_params(rom: Option<Rom>, args: CmdArgs) -> Result<Self> {
        let mut exe = Executer::new();
        exe.args = args;

        if let Some(rom) = rom {
            exe.set_rom(rom);
            exe.hard_reset();
        }

        Ok(Self { exe })
    }

    fn run(rom: Rom, args: CmdArgs) {
        let win_size = Vector::new(255, 240);
        run_with("RUSTNESS", win_size, Settings::default(), || {
            Self::new_with_params(Some(rom), args)
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

        let keys = window.keyboard();

        let keybits: u32 = [
            (Key::A, PadButton::Left),
            (Key::S, PadButton::Down),
            (Key::W, PadButton::Up),
            (Key::D, PadButton::Right),
            (Key::K, PadButton::A),
            (Key::J, PadButton::B),
            (Key::G, PadButton::Select),
            (Key::H, PadButton::Start),
        ]
        .into_iter()
        .filter(|(k, _pad)| keys[*k].is_down())
        .fold(0_u32, |acc, (_k, pad)| acc | (1 << (*pad as u32)));

        // TODO: joypad1 の入力も必要
        self.exe.set_joypad_keybits(0, keybits);

        // pause
        if keys[Key::Space] == ButtonState::Pressed {
            self.exe.args.pause = !self.exe.args.pause;
        }

        // debug_mode: toggle with keys
        [
            (Key::Key0, 0),
            (Key::Key1, 1),
            (Key::Key2, 2),
            (Key::Key3, 3),
            (Key::Key4, 4),
        ]
        .into_iter()
        .filter(|(key, _v)| keys[*key] == ButtonState::Pressed)
        .nth(0)
        .map(|(_key, v)| {
            let nv = if let Some(pv) = self.exe.args.debug_mode {
                if pv == *v {
                    None
                } else {
                    Some(*v)
                }
            } else {
                Some(*v)
            };
            self.exe.args.debug_mode = nv;
        });

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // 本来 update() で呼び出すべきかもしれないが、
        // ここに書かないとキーイベントなどがうまくとれない.
        if !self.exe.args.pause {
            let frame_count = self.exe.get_frame_count();
            loop {
                self.exe.execute();
                if self.exe.get_frame_count() > frame_count {
                    break;
                }
            }
        }

        if self.exe.args.debug_level > 0 {
            println!("update frame: {}", self.exe.get_frame_count());
        }

        {
            let fb = if let Some(debug_mode) = self.exe.args.debug_mode {
                self.exe.get_debug_frame_buffer(debug_mode)
            } else {
                self.exe.get_frame_buffer()
            };

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

    let rom_path = matches.value_of("rom").unwrap_or(
        //"static/sample1/sample1.nes"
        //"static/roms/giko005.nes"
        //"static/roms/giko008.nes"
        //"static/roms/giko009.nes"
        //"static/roms/giko010b.nes"

        //"static/roms/giko016.nes" // NG
        //"static/roms/falling.nes" // NG
        //"static/roms/nestest.nes" // OK
        //"static/local/Super_mario_brothers.nes" // NG
        "static/roms/giko011.nes", // NG
    );

    let args = CmdArgs {
        debug_level,
        rom_path: rom_path.to_string(),
        pause: false,
        debug_mode: None,
    };

    if false || args.debug_level > 0 {
        println!("rom_path: [{}]", rom_path);
        println!("debug-level: {}", debug_level);
    }

    let rom = Rom::load_image(rom_path);
    Rustness::run(rom, args);
}
