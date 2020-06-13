mod normal_color;

pub use crate::hittables::{Direction, Float, HitRecord, Point, Ray};
pub use crate::rgb::RGB;
pub use normal_color::NormalColor;

pub trait Material {
    fn get_color(&self, hit: &HitRecord) -> RGB;
}
