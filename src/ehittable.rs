
use crate::{
    Point,
    ray::Ray,
    hittable::HitRecord,
    hittable::Sphere,
    hittable_list::HittableList,
    material::Material,
};

pub enum EHittable {
    Sphere(Sphere),
    HittableList(HittableList),
}

impl From<Sphere> for EHittable {
    fn from(sphere: Sphere) -> EHittable {
        EHittable::Sphere(sphere) 
    }
}

impl From<HittableList> for EHittable {
    fn from(hl: HittableList) -> EHittable {
        EHittable::HittableList(hl) 
    }
}

impl EHittable {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            EHittable::Sphere(sphere) => {
                sphere.hit(r, t_min, t_max, rec)
            }
            EHittable::HittableList(hittablelist) => {
                hittablelist.hit(r, t_min, t_max, rec)
            }
        }
    }
}

impl Default for EHittable {
    fn default() -> Self {
        EHittable::Sphere(Default::default())
    }
}