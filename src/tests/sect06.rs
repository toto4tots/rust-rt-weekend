
use crate::{
    vec3,
    vec3::Point,
    vec3::Color,
    vec3::Vec3,
    ray::Ray,
    hittable,
    hittable::Sphere,
    hittable::Hittable,
    hittable::HitRecord,
    hittable_list,
    hittable_list::HittableList
};

#[test]
pub fn hittable() {
    {
        let r = Ray::new(Point::new(0, 1, -5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let mut rec = hittable::HitRecord {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0, front_face: false};
        assert_eq!(true, s.hit(r, 5.0, 5.0, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 2, -5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let mut rec = hittable::HitRecord {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0, front_face: false};
        assert_eq!(false, s.hit(r, -100.0, 100.0, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 0), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let mut rec = hittable::HitRecord {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0, front_face: false};
        assert_eq!(true, s.hit(r, -1.0, 1.0, &mut rec));
        assert_eq!(false, s.hit(r, 1.5, 2.0, &mut rec));
        assert_eq!(false, s.hit(r, -2.0, -1.5, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let mut rec = hittable::HitRecord {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0, front_face: false};
        assert_eq!(true, s.hit(r, -6.0, -4.0, &mut rec));
        assert_eq!(true, s.hit(r, -8.0, -5.0, &mut rec));
        assert_eq!(true, s.hit(r, -7.0, -3.0, &mut rec));
        assert_eq!(false, s.hit(r, -2.0, -1.5, &mut rec));
    }

}

#[test]
pub fn hittable_list() {
    {
        let r = Ray::new(Point::new(0, 1, -5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let mut rec = HitRecord::new();
        assert_eq!(true, s.hit(r, 5.0, 5.0, &mut rec));
    }    
    {
        let s = Sphere::new();
        let world = HittableList::new(vec![Box::new(s)]);
        
        // println!("{:?}", world);


    }
}
