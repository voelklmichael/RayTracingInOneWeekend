mod fuzzy_metal;
mod lambertian;
mod specular;

pub use crate::hittables::{Direction, Float, HitRecord, Point, Ray};
pub use crate::rgb::RGB;
pub use fuzzy_metal::FuzzyMetal;
pub use lambertian::Lambertian;
pub use specular::Specular;

pub trait Material
where
    Self: PerfectHash,
{
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> bool;
}

pub trait PerfectHash {
    fn hash(&self) -> u128;
}
impl<T> PerfectHash for T
where
    T: std::fmt::Debug,
{
    fn hash(&self) -> u128 {
        let bytes = format!("{:?}", self).into_bytes();
        bytes.len() as u128 + bytes.into_iter().map(|x| x as u128).sum::<u128>()
    }
}
