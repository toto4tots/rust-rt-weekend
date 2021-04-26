
use crate::vec3::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::random_unit_vector;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
}

impl Material {
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
                let fuzz = if *f < 1.0 {*f} else {1.0};
                let reflected = r_in.direction.unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.p, reflected + random_unit_vector().scale(fuzz));
                *attenuation = *albedo;
                scattered.direction.dot(rec.normal) > 0.0
            }
        }
    }


}

impl Default for Material {
    fn default() -> Self { Material::Lambertian(Color::new(0, 0, 0)) }
}
