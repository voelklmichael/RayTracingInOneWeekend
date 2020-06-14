pub use super::hittables::{Direction, Float, HitRecord, Hittable, Point, Ray};
pub use super::material::Material;
pub use super::rgb::RGB;
pub struct Scene {
    materials: Vec<Box<dyn Material>>,
    materials_hashes: Vec<u128>,
    objects: Vec<(Box<dyn Hittable>, usize)>,
    background: RGB,
}
impl Scene {
    pub fn new(background: RGB) -> Self {
        Self {
            materials: Vec::new(),
            materials_hashes: Vec::new(),
            objects: Vec::new(),
            background,
        }
    }
    pub fn push(&mut self, h: Box<dyn Hittable>, m: Box<dyn Material>) {
        let hash = m.hash();
        let index = if let Some(index) = self.materials_hashes.iter().position(|&h| h == hash) {
            index
        } else {
            let index = self.materials_hashes.len();
            self.materials.push(m);
            self.materials_hashes.push(hash);
            index
        };
        self.objects.push((h, index));
    }
    pub fn clear(&mut self, new_background: RGB) {
        self.objects.clear();
        self.materials.clear();
        self.materials_hashes.clear();
        self.background = new_background;
    }
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        mut distance_max: Float,
    ) -> Option<(HitRecord, &dyn Material)> {
        let mut hit = None;
        for (hittable, material_index) in self.objects.iter() {
            if let Some(h) = hittable.hit(ray, distance_min, distance_max) {
                distance_max = h.distance;
                hit = Some((h, self.materials[*material_index].as_ref()));
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
