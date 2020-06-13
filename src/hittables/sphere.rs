use super::{hit_record::HitRecord, Float, Hittable, Point, Ray};
pub struct Sphere {
    center: Point,
    radius: Float,
}
impl Sphere {
    pub fn new(center: Point, radius: Float) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let oc = ray.position().clone() - self.center.clone();
        let a = ray.direction().l2_norm_squared();
        let half_b = oc.clone().dot(&ray.direction());
        let c = oc.l2_norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < distance_max && temp > distance_min {
                let distance = temp;
                let position = ray.next(distance);
                let mut outward_normal = (position.clone() - self.center.clone()) / self.radius;
                let is_hitting_front_outward =
                    HitRecord::is_hitting_front_outward(ray, &mut outward_normal);
                Some(HitRecord {
                    position,
                    outward_normal,
                    distance,
                    is_hitting_front_outward,
                })
            } else {
                let temp = (-half_b + root) / a;
                if temp < distance_max && temp > distance_min {
                    let distance = temp;
                    let position = ray.next(distance);
                    let mut outward_normal = (position.clone() - self.center.clone()) / self.radius;
                    let is_hitting_front_outward =
                        HitRecord::is_hitting_front_outward(ray, &mut outward_normal);
                    Some(HitRecord {
                        position,
                        outward_normal,
                        distance,
                        is_hitting_front_outward,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
