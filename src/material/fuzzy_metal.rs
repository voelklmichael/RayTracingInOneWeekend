use super::{Direction, Float, HitRecord, Material, Ray, RGB};

#[derive(Debug)]
pub struct FuzzyMetal {
    albedo: RGB,
    fuzziness: Float,
}
impl Material for FuzzyMetal {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> bool {
        ray.adjust_brightness(&self.albedo);
        let new_direction = ray.direction().reflect(&hit.outward_normal);
        let new_direction =
            new_direction.add(&(Direction::random_in_unit_sphere() * self.fuzziness));
        *ray.direction_mut() = new_direction;
        true
    }
}

impl FuzzyMetal {
    pub fn new(albedo: RGB, fuzziness: Float) -> Self {
        Self { albedo, fuzziness }
    }
}
