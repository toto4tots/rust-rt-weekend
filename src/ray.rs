
use crate::{
    vec3,
    vec3::Point,
    vec3::Color,
    vec3::Vec3,
    ehittable::Hittable,
    hittable::HitRecord,
};

use std::f64::INFINITY;

pub fn ray_color(r: Ray, world: &Hittable, depth: i64) -> vec3::Vec3 {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color::new(0, 0, 0);
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0, 0, 0);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1, 1, 1).scale(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3
}

impl Ray {
    pub fn new<I: Into<Vec3>>(orig0: I, dir0: I) -> Self {
        let orig = orig0.into();
        let dir = dir0.into();
        Ray {
            origin: orig,
            direction: dir
        }
    }

    pub fn at<N: Into<f64>>(&self, t0: N) -> Vec3 {
        let t = t0.into();
        self.origin + self.direction.scale(t)
    }
}
