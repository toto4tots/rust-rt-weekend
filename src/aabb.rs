
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
        f64::min(box0.maximum.x(), box1.maximum.x()),
        f64::min(box0.maximum.y(), box1.maximum.y()),
        f64::min(box0.maximum.z(), box1.maximum.z()),
    );
    AABB::new(small, big)
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AABB {
    pub minimum: Point,
    pub maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        AABB {
            minimum,
            maximum
        }
    }

    pub fn hit(&self, r: Ray, t_min: &mut f64, t_max: &mut f64) -> bool {
        for a in 0..3 {
            let invd = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * invd;
            let mut t1 = (self.maximum[a] - r.origin[a]) * invd;
            if invd < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            *t_min = if t0 > *t_min { t0 } else { *t_min };
            *t_max = if t1 > *t_max { t1 } else { *t_max };
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}
