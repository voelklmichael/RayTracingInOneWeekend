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
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }
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
    pub fn gamma2(&self) -> RGBu8 {
        let Self { r, g, b } = self;
        RGBu8 {
            r: Self::round_(&r.sqrt()),
            g: Self::round_(&g.sqrt()),
            b: Self::round_(&b.sqrt()),
        }
    }
    pub fn average(&self, other: &Self) -> Self {
        Self {
            r: 0.5 * (self.r + other.r),
            g: 0.5 * (self.g + other.g),
            b: 0.5 * (self.b + other.b),
        }
    }
    pub fn white() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
    pub fn black() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
    pub fn normalize(&mut self, n: usize) {
        let m = 1. / n as Float;
        self.r *= m;
        self.g *= m;
        self.b *= m;
    }
}
impl std::ops::AddAssign<RGB> for RGB {
    fn add_assign(&mut self, rhs: RGB) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
impl std::ops::Add<RGB> for RGB {
    type Output = RGB;
    fn add(self, rhs: RGB) -> RGB {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::AddAssign<&RGB> for RGB {
    fn add_assign(&mut self, rhs: &RGB) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
impl std::ops::MulAssign<&RGB> for RGB {
    fn mul_assign(&mut self, rhs: &RGB) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}
