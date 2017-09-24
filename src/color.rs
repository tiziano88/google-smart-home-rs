extern crate rgb;

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

const P: f32 = 0.9;

pub trait ColorFunc: Send {
    fn step(&self, t: u64, current: &[rgb::RGB8]) -> [rgb::RGB8; 16];
    fn color(&self) -> rgb::RGB8;
}

pub struct SolidColor {
    pub c: rgb::RGB8,
}

impl ColorFunc for SolidColor {
    fn step(&self, t: u64, current: &[rgb::RGB8]) -> [rgb::RGB8; 16] {
        let mut pixels = [BLACK; 16];
        for i in 0..pixels.len() {
            pixels[i] = mean(current[i], self.c, P);
        }
        pixels
    }

    fn color(&self) -> rgb::RGB8 {
        self.c
    }
}

fn mean(x: rgb::RGB8, y: rgb::RGB8, p: f32) -> rgb::RGB8 {
    rgb::RGB8 {
        r: (x.r as f32 * p + y.r as f32 * (1.0 - p)) as u8,
        g: (x.g as f32 * p + y.g as f32 * (1.0 - p)) as u8,
        b: (x.b as f32 * p + y.b as f32 * (1.0 - p)) as u8,
    }
}
