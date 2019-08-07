#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
pub struct Rom {
    pub bin: Vec<u8>,
}

impl Rom {
    pub fn dummy() -> Self {
        Self { bin: Vec::new() }
    }

    pub fn load_image(filename: String) -> Rom {
        let fp = File::open(filename).unwrap();
        let mut bin: Vec<u8> = Vec::with_capacity(1024 * 1024);
        let mut reader = std::io::BufReader::new(fp);
        reader.read_to_end(&mut bin).unwrap();
        Rom { bin }
    }

    fn get_signature(&self) -> [u8; 4] {
        let mut ret = [0u8; 4];
        ret.copy_from_slice(&self.bin[0..4]);
        ret
    }

    fn get_header(&self) -> &[u8] {
        &self.bin[0..16]
    }

    pub fn get_bytes_of_prg(&self) -> usize {
        self.get_header()[4] as usize * 16 * 1024
    }

    pub fn get_bytes_of_chr(&self) -> usize {
        self.get_header()[5] as usize * 8 * 1024
    }

    pub fn get_prg(&self) -> &[u8] {
        let start = 0x10;
        let end = start + self.get_bytes_of_prg();
        &self.bin[start..end]
    }
    pub fn get_chr(&self) -> &[u8] {
        let start = 0x10 + self.get_bytes_of_prg();
        let end = start + self.get_bytes_of_chr();
        &self.bin[start..end]
    }

    pub fn write_png(&self, path: &Path) {
        let chr = self.get_chr();

        let nblocks = chr.len() / 16;
        let w = 8 * 64; // 512
        let h = (nblocks / 64) * 8;

        println!(
            "write_png: {}, nblocks: {}, w,h: {}, {}",
            chr.len(),
            nblocks,
            w,
            h
        );

        let nbuf: usize = w * h * 4;
        let mut buf: Vec<u8> = vec![0u8; nbuf];

        // 一ブロックは 16 byte
        for (idx_block, chunk) in chr.chunks(16).enumerate() {
            let xb = idx_block % 64;
            let yb = idx_block / 64;
            let x_base = xb * 8;
            let y_base = yb * 8;

            for iy in 0..8 {
                for ix in 0..8 {
                    let lines = (chunk[iy], chunk[iy + 8]);
                    let rx = 7 - ix; // reverse endian
                    let a = lines.0 >> rx & 0x1;
                    let b = lines.1 >> rx & 0x1;
                    let bit = a | (b << 1);

                    assert!(bit < 4);
                    let c = match bit {
                        0 => 0x00,
                        1 => 0x55,
                        2 => 0xaa,
                        3 => 0xff,
                        _ => 0xff,
                    };

                    let y = iy + y_base;
                    let x = ix + x_base;
                    let dst_idx = y * w * 4 + x * 4;
                    buf[dst_idx + 0] = c;
                    buf[dst_idx + 1] = c;
                    buf[dst_idx + 2] = c;
                    buf[dst_idx + 3] = 0xff;
                }
            }
        }

        image::save_buffer(path, &buf, w as u32, h as u32, image::RGBA(8)).unwrap();
    }
}

#[test]
fn test_rom() {
    let rom = Rom::load_image("static/sample1/sample1.nes".to_string());
    let sig = rom.get_signature();
    assert_eq!(sig, "NES\u{1a}".as_bytes());

    let header = rom.get_header();
    assert_eq!(16, header.len());

    assert_eq!(rom.get_bytes_of_prg(), rom.get_prg().len());
    assert_eq!(rom.get_bytes_of_chr(), rom.get_chr().len());
}
