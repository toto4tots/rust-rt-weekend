
use crate::vec3::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::random_unit_vector;
use crate::vec3::Vec3;
use crate::rtweekend::random_float;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

impl Material {
    pub fn metal(color: Color, fuzz: f64) -> Self {
        Self::Metal(color, if fuzz < 1.0 {fuzz} else {1.0})
    }

    pub fn scatter(
    &self,
    r_in: Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = rec.normal + random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                return true;
            },
            Material::Metal(albedo, f) => {
                let reflected = r_in.direction.unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.p, reflected + random_unit_vector().scale(*f));
                *attenuation = *albedo;
                scattered.direction.dot(rec.normal) > 0.0
            }
            Material::Dielectric(ir) => {
                *attenuation = Color::new(1, 1, 1);
                let refraction_ratio = if rec.front_face {1.0 / ir} else {*ir};
                let unit_direction = r_in.direction.unit_vector();
                let cos_theta = (unit_direction).scale(-1.0).dot(rec.normal).min(1.0);
                let sine_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sine_theta > 1.0;
                let mut direction: Vec3 = Default::default();

                if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
                    direction = unit_direction.reflect(rec.normal);
                } else {
                    direction = unit_direction.refract(rec.normal, refraction_ratio);
                }
                *scattered = Ray::new(rec.p, direction);
                true
            }
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Default for Material {
    fn default() -> Self { Material::Lambertian(Color::new(0, 0, 0)) }
}
