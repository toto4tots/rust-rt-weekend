
use crate::{
    Point,
    ray::Ray,
    hittable::HitRecord,
    hittable::Sphere,
    hittable_list::HittableList,
    material::Material,
};

pub enum Hittable {
    Sphere(Sphere),
    HittableList(HittableList),
}

impl From<Sphere> for Hittable {
    fn from(sphere: Sphere) -> Hittable {
        Hittable::Sphere(sphere) 
    }
}

impl From<HittableList> for Hittable {
    fn from(hl: HittableList) -> Hittable {
        Hittable::HittableList(hl) 
    }
}

impl Hittable {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(sphere) => {
                sphere.hit(r, t_min, t_max, rec)
            }
            Hittable::HittableList(hittablelist) => {
                hittablelist.hit(r, t_min, t_max, rec)
            }
        }
    }
}

impl Default for Hittable {
    fn default() -> Self {
        Hittable::Sphere(Default::default())
    }
}
