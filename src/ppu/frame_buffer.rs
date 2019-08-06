extern crate image;
use std::path::Path;

use crate::ppu::RGB;

pub struct FrameBuffer {
    w: u32,
    h: u32,
    buf: Vec<u8>, // RGBs
}

impl FrameBuffer {
    fn new(w: u32, h: u32) -> Self {
        let s = w * h * 3;
        FrameBuffer {
            w,
            h,
            buf: vec![0; s as usize],
        }
    }

    pub fn set_pixel(&mut self, pos: (u32, u32), c: &RGB) {
        let p = 3 * (self.w * pos.1 + pos.0) as usize;
        self.buf[p + 0] = c.0;
        self.buf[p + 1] = c.1;
        self.buf[p + 2] = c.2;
    }

    fn save_as_png<P: AsRef<Path>>(&self, path: P) {
        image::save_buffer(path, &self.buf, self.w, self.h, image::RGB(8)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_buffer() {
        let fb = FrameBuffer::new(320, 240);
        assert_eq!(fb.w, 320);
        assert_eq!(fb.h, 240);
        assert_ne!(fb.buf.len(), 0);
    }

    #[test]
    fn test_save_as_png() {
        let mut fb = FrameBuffer::new(32, 24);
        fb.set_pixel((0, 0), &RGB(0xff, 0, 0));
        fb.set_pixel((1, 0), &RGB(0xff, 0, 0));
        fb.set_pixel((2, 0), &RGB(0xff, 0, 0));
        fb.set_pixel((0, 1), &RGB(0, 0xff, 0));
        fb.set_pixel((1, 1), &RGB(0, 0, 0xff));
        fb.set_pixel((2, 1), &RGB(0, 0xff, 0xff));
        fb.save_as_png("screenshot/ss_test00.png");
    }
}
