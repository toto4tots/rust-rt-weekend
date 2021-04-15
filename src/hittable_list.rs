use crate::{
    hittable::Hittable,
    hittable::HitRecord,
    ray::Ray,
};

use std::fmt;

// #[derive(Debug)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new (objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList {
            objects: objects
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

// impl fmt::Display for HittableList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[");
//         for obj in &self.objects {
//             write!(f, "{:?}", obj);
//         }
//         write!(f, "]")
//     }

// }

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}


