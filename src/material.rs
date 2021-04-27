
use crate::vec3::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::random_unit_vector;

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
                // let fuzz = if *f < 1.0 {*f} else {1.0};
                let reflected = r_in.direction.unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.p, reflected + random_unit_vector().scale(*f));
                *attenuation = *albedo;
                scattered.direction.dot(rec.normal) > 0.0
            }
            Material::Dielectric(ir) => {
                *attenuation = Color::new(1, 1, 1);
                let refraction_ratio = if rec.front_face {1.0 / ir} else {*ir};
                let unit_direction = r_in.direction.unit_vector();
                let refracted = unit_direction.refract(rec.normal, refraction_ratio);
                *scattered = Ray::new(rec.p, refracted);
                true
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self { Material::Lambertian(Color::new(0, 0, 0)) }
}
