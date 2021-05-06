
use crate::vec3::Color;
use std::rc::Rc;
use crate::material::Material;
use crate::{
    vec3::Vec3,
    vec3::Point,
    ray::Ray,
    hittable_list::HittableList,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self { Default::default() }
    
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal.scale(-1.0)};
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.mag_squared(); // dot product on itself
        let half_b = oc.dot(r.direction);
        let c = oc.mag_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the accepetable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root  {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center).scale(1.0 / self.radius);
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material;

        true
    }    
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point::new(0, 0, 0), 
            radius: 1.0,
            material: Material::Lambertian(Color::new(0, 0, 0)),
        }
    }
}
