
use crate::{
    vec3::Vec3,
    vec3::Point,
    ray::Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            // dummy values
            p: Point::new(0, 0, 0),
            normal: Vec3::new(0, 0, 0),
            t: 0.0, front_face: false
        }
    }
    
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal.scale(-1.0)};
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0, 0, 0),
            radius: 1.0
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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

        true
    }
}
