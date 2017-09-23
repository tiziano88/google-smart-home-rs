extern crate rgb;

trait ColorFunc {
    fn step(&self, t: u64, current: &[rgb::RGB8; 16]) -> [rgb::RGB8; 16];
}

struct SolidColor {
    c: rgb::RGB8,
}

impl ColorFunc for SolidColor {
    fn step(&self, t: u64, current: &[rgb::RGB8; 16]) -> [rgb::RGB8; 16] {
        [self.c; 16]
    }
}
