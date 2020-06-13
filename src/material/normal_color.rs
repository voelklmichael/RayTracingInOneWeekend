use super::{HitRecord, Material, RGB};
pub struct NormalColor {
    background: RGB,
}
impl Material for NormalColor {
    fn get_color(&self, hit: &HitRecord) -> RGB {
        let (r, g, b) = hit.outward_normal.extract();
        self.background.average(&RGB { r, g, b })
    }
}

impl NormalColor {
    pub fn new(background: RGB) -> Self {
        Self { background }
    }
}
