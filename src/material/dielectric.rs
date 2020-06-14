use super::{Direction, Float, HitRecord, Material, Ray};

#[derive(Debug)]
pub struct Dielectric {
    refractive_index: Float,
}
impl Material for Dielectric {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> bool {
        let etai_over_etat = if hit.is_hitting_front_outward {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let uv = ray.direction() * (1. / ray.direction().length());
        let cos_theta = {
            let a = -uv.dot(&hit.outward_normal);
            if a < 1. {
                a
            } else {
                1.
            }
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let new_direction =
            if etai_over_etat * sin_theta > 1.0 || crate::types::generate_random() < reflect_prob {
                uv.reflect(&hit.outward_normal)
            } else {
                Direction::refract(&uv, &hit.outward_normal, etai_over_etat)
            };

        *ray.direction_mut() = new_direction;
        true
    }
}

impl Dielectric {
    pub fn new(refractive_index: Float) -> Self {
        Self { refractive_index }
    }
}

fn schlick(cos_theta: Float, ref_idx: Float) -> Float {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cos_theta).powi(5);
}
