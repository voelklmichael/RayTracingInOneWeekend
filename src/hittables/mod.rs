mod hit_record;
mod sphere;

pub use super::ray::{Direction, Float, Point, Ray};
pub use hit_record::HitRecord;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
    ) -> Option<hit_record::HitRecord>;
}
