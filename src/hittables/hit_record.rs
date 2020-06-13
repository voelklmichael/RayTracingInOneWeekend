use crate::ray::{Direction, Float, Point, Ray};
pub struct HitRecord {
    pub position: Point,
    pub outward_normal: Direction,
    pub distance: Float,
    pub is_hitting_front_outward: bool,
}

impl HitRecord {
    pub fn is_hitting_front_outward(ray: &Ray, outward_normal: &mut Direction) -> bool {
        let is_hitting_front_outward = ray.direction().dot(&outward_normal) < 0.;
        if is_hitting_front_outward {
            true
        } else {
            outward_normal.invert();
            false
        }
    }
}
