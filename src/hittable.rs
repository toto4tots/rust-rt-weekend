
use crate::{
    vec3::Vec3,
    vec3::Point,
    ray::Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct Hit_Record {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: Hit_Record) -> bool;
}

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0, 0, 0),
            radius: 1.0
        }
    }
    // pub fn new(cen: Point, r: f64) -> Self {
    //     Sphere {
    //         center: Point::new(0, 0, 0),
    //         radius: 1.0
    //     }
    // }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, mut rec: Hit_Record) -> bool {
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
        rec.normal = (rec.p - self.center).scale(1.0 / self.radius);

        true
    }
}


