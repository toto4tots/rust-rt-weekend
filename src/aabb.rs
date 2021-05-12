
use crate::{
    vec3::Point,
    ray::Ray
};

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point::new(
        f64::min(box0.minimum.x(), box1.minimum.x()),
        f64::min(box0.minimum.y(), box1.minimum.y()),
        f64::min(box0.minimum.z(), box1.minimum.z()),
    );
    let big = Point::new(
        f64::max(box0.maximum.x(), box1.maximum.x()),
        f64::max(box0.maximum.y(), box1.maximum.y()),
        f64::max(box0.maximum.z(), box1.maximum.z()),
    );
    AABB::new(small, big)
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AABB {
    pub minimum: Point,
    pub maximum: Point,
}

impl AABB {
    pub fn new(a: Point, b: Point) -> Self {
        let small = Point::new(
            f64::min(a.x(), b.x()),
            f64::min(a.y(), b.y()),
            f64::min(a.z(), b.z()),
        );
        let big = Point::new(
            f64::max(a.x(), b.x()),
            f64::max(a.y(), b.y()),
            f64::max(a.z(), b.z()),
        );        
        AABB {
            minimum: small,
            maximum: big,
        }
    }

    pub fn hit(&self, r: Ray, t_min0: f64, t_max0: f64) -> bool {
        let mut t_min = t_min0;
        let mut t_max = t_max0;
        for a in 0..3 {
            let invd = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * invd;
            let mut t1 = (self.maximum[a] - r.origin[a]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}
