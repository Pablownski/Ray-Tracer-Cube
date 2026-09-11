pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_correctly_sized_buffer() {
        let fb = Framebuffer::new(640, 360);
        assert_eq!(fb.buffer.len(), 640 * 360);
    }

    #[test]
    fn clear_fills_every_pixel() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(0x123456);
        assert!(fb.buffer.iter().all(|&p| p == 0x123456));
    }

    #[test]
    fn set_pixel_writes_expected_index() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel(3, 2, 0xff0000);
        assert_eq!(fb.buffer[2 * 10 + 3], 0xff0000);
    }

    #[test]
    fn set_pixel_out_of_bounds_is_ignored() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(100, 100, 0xffffff);
        assert!(fb.buffer.iter().all(|&p| p == 0));
    }
}
