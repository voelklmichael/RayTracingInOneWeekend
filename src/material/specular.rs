use super::{HitRecord, Material, Ray, RGB};

#[derive(Debug)]
pub struct Specular {
    albedo: RGB,
}
impl Material for Specular {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> bool {
        ray.adjust_brightness(&self.albedo);
        let new_direction = ray.direction().reflect(&hit.outward_normal);
        *ray.direction_mut() = new_direction;
        true
    }
}

impl Specular {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}
