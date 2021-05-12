
use std::cmp::Ordering;
use crate::{
    ray::Ray,
    aabb::AABB,
    aabb::surrounding_box,
    hittable::HitRecord,
    hittable::Hittable,
    rtweekend::random_int_with_range
};

pub fn box_compare(a: &Hittable, b: &Hittable, axis: i32) -> Ordering {
    let box_a: AABB = Default::default();
    let box_b: AABB = Default::default();

    if a.bounding_box().is_none() || b.bounding_box().is_none() {
        panic!("No bounding box in bvh_node constructor.");
    }
    box_a.minimum.e[axis as usize].partial_cmp(&box_b.minimum.e[axis as usize]).unwrap()
    // box_a.minimum.e[axis as usize] < box_b.minimum.e[axis as usize]
}

pub struct BvhNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    pub b: AABB
}

impl BvhNode {
    pub fn from(mut objects: Vec<Hittable>) -> Hittable {
        let end = objects.len();
        Self::new(&mut objects, 0, end, 0)
    }
    fn new(objects: &mut Vec<Hittable>, start: usize, end: usize, axis: i32) -> Hittable {
        let object_span = end - start;
        if object_span == 1 {
            std::mem::replace(&mut objects[start], Default::default())
        } else {
            let next_axis = (axis + 1) % 3;
            let comparator = | a: &Hittable, b: &Hittable | box_compare(a, b, axis);
            &objects[start..end].sort_by(|a, b| comparator(&a, &b));
            let mid = start + object_span / 2;
            let left = BvhNode::new(objects, start, mid, next_axis);
            let right = BvhNode::new(objects, mid, end, next_axis);
            let lbox = left.bounding_box();
            let rbox = right.bounding_box();
            if lbox.is_none() || rbox.is_none() {
                panic!("No bounding box is bvh node constructor");
            }
            let b = surrounding_box(lbox.unwrap(), rbox.unwrap());
            let parent = Self {
                left: left.into(),
                right: right.into(),
                b,
            };
            Hittable::BvhNode(parent)         
        }
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.b.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
}
