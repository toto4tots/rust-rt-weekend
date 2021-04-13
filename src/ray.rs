
use crate::{
    vec3,
    vec3::Point,
    vec3::Color,
    vec3::Vec3,
};

pub fn ray_color(r: &Ray) -> vec3::Vec3 {
    let t = hit_sphere(Point::new(0, 0, -1), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0, 0, -1)).unit_vector();
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0).scale(0.5);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1, 1, 1).scale(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
}

pub fn hit_sphere(center: Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

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
