struct Color(u8, u8, u8);

struct FrameBuffer {
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

    fn set_pixel(&mut self, x: u32, y: u32, c: &Color) {
        let p = 3 * (self.w * y + x) as usize;
        self.buf[p + 0] = c.0;
        self.buf[p + 1] = c.1;
        self.buf[p + 2] = c.2;
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

    fn test_save_in_png() {
        let mut fb = FrameBuffer::new(320, 240);
        fb.set_pixel(0, 0, &Color(0xff, 0, 0));
        fb.set_pixel(1, 0, &Color(0xff, 0, 0));
        fb.set_pixel(2, 0, &Color(0xff, 0, 0));
    }

}
