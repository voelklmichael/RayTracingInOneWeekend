pub use super::hittables::{Direction, Float, HitRecord, Hittable, Point, Ray};
pub use super::material::Material;
pub use super::rgb::RGB;
pub struct Scene {
    objects: Vec<(Box<dyn Hittable>, Box<dyn Material>)>,
    background: RGB,
}
impl Scene {
    pub fn new(background: RGB) -> Self {
        Self {
            objects: Vec::new(),
            background,
        }
    }
    pub fn push(&mut self, h: Box<dyn Hittable>, m: Box<dyn Material>) {
        self.objects.push((h, m));
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        mut distance_max: Float,
    ) -> Option<(HitRecord, &dyn Material)> {
        let mut hit = None;
        for (hittable, material) in self.objects.iter() {
            if let Some(h) = hittable.hit(ray, distance_min, distance_max) {
                distance_max = h.distance;
                hit = Some((h, material.as_ref()));
            }
        }
        hit
    }
    pub fn background(&self, ray: &Ray) -> RGB {
        let dir = ray.direction().unit_vector();
        let t = 0.5 * (dir.y() + 1.);
        let s = 1. - t;
        let RGB { r, g, b } = self.background;
        RGB {
            r: s + r * t,
            g: s + g * t,
            b: s + b * t,
        }
    }
}
