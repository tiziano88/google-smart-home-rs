extern crate rgb;

pub trait ColorFunc {
    fn step(&self, t: u64, current: &[rgb::RGB8; 16]) -> [rgb::RGB8; 16];
    fn color(&self) -> rgb::RGB8;
}

pub struct SolidColor {
    pub c: rgb::RGB8,
}

impl ColorFunc for SolidColor {
    fn step(&self, t: u64, current: &[rgb::RGB8; 16]) -> [rgb::RGB8; 16] {
        [self.c; 16]
    }

    fn color(&self) -> rgb::RGB8 {
        self.c
    }
}
