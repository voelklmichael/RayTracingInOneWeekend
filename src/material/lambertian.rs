use super::{Direction, HitRecord, Material, Ray, RGB};

#[derive(Debug)]
pub struct Lambertian {
    albedo: RGB,
}
impl Material for Lambertian {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> bool {
        ray.adjust_brightness(&self.albedo);
        let new_direction = Direction::random_lambertian(&hit.outward_normal);
        *ray.direction_mut() = new_direction;
        true
    }
}

impl Lambertian {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}
