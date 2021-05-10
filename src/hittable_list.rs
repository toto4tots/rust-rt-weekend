use crate::{
    hittable::HitRecord,
    hittable::Hittable,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Hittable>
}

impl HittableList {
    pub fn new (objects: Vec<Hittable>) -> Self {
        HittableList { objects }
    }

    pub fn empty(&self) -> bool {
        self.objects.len() == 0
    }

    pub fn add(&mut self, obj: Hittable) {
        self.objects.push(obj);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
