
use crate::{
    vec3,
    vec3::Point,
    vec3::Color,
    vec3::Vec3,
    ray::Ray,
    hittable,
    hittable::Sphere,
    hittable::Hittable
};

#[test]
pub fn hittable() {
    {
        let r = Ray::new(Point::new(0, 1, -5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let rec = hittable::Hit_Record {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0};
        assert_eq!(true, s.hit(r, 5.0, 5.0, rec));
    }
    {
        let r = Ray::new(Point::new(0, 2, -5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let rec = hittable::Hit_Record {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0};
        assert_eq!(false, s.hit(r, -100.0, 100.0, rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 0), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let rec = hittable::Hit_Record {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0};
        assert_eq!(true, s.hit(r, -1.0, 1.0, rec));
        assert_eq!(false, s.hit(r, 1.5, 2.0, rec));
        assert_eq!(false, s.hit(r, -2.0, -1.5, rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 5), Vec3::new(0, 0, 1));
        let s = Sphere::new();
        let rec = hittable::Hit_Record {p: Point::new(0, 0, 0), normal: Vec3::new(0, 0, 0), t: 0.0};
        assert_eq!(true, s.hit(r, -6.0, -4.0, rec));
        assert_eq!(true, s.hit(r, -8.0, -5.0, rec));
        assert_eq!(true, s.hit(r, -7.0, -3.0, rec));
        assert_eq!(false, s.hit(r, -2.0, -1.5, rec));
    }

}
