pub use crate::types::Float;

#[derive(Debug)]
pub struct RGB {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

#[derive(Debug)]
pub struct RGBu8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    fn round_(f: &Float) -> u8 {
        let f = *f * 255.999;
        if f < 256.0 {
            f as u8
        } else {
            255
        }
    }
    pub fn round(&self) -> RGBu8 {
        let Self { r, g, b } = self;
        RGBu8 {
            r: Self::round_(r),
            g: Self::round_(g),
            b: Self::round_(b),
        }
    }
}
