use crate::hittable::HitRecord;
use crate::vec3::Color;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::vec3::Point;
use crate::hittable::Sphere;
use crate::hittable::Hittable;
use crate::drawutils::Canvas;
use std::f64::INFINITY;
use crate::{
    rtweekend::random_float,
    rtweekend::random_float_with_range,
};
use crate::{
    material,
    material::Material
};
use crate::{
    bvh::BvhNode
};

fn create_spheres() -> Vec<Hittable> {
    // World
    let material_ground = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = Material::Dielectric(1.5);
    let material_right = Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0);

    let s1 = Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground
    );
    let s2 = Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center
    );
    let s3 = Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    );
    let s4 = Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right
    );
    let s5 = Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left
    );

    vec![
            s1.into(), 
            s2.into(),
            s3.into(),
            s4.into(),
            s5.into(),
        ]   
}

#[test]
pub fn world_hit() {
    let world: Hittable = HittableList::new(create_spheres()).into();
    {
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(-1.7772153609911319, 0.00517260924540619, -1.0));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.9048564458630389, 0.0026335968731590856, -0.5091428229375708);
        let norm = Vec3::new(0.19028710827392215, 0.005267193746318171, 0.9817143541248583);
        let t = 0.5091428229375708;
        let front_face = true;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);

    }
    {
        let r = Ray::new(Vec3::new(-0.9048564458630389, 0.0026335968731590856, -0.5091428229375708), Vec3::new(-0.6627807911795999, -0.0005725727441804, -0.7488132577644417));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.98551439270837, 0.00256391690871263, -0.6002705996408596);
        let norm = Vec3::new(0.036214018229074885, 0.006409792271781576, 0.9993235008978509);
        let t = 0.1216962650679394;
        let front_face = false;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);
    }

    {
        let r = Ray::new(Vec3::new(-0.904730488748051, 0.0011753099041315778, -0.5091615959682413), Vec3::new(-0.6628487795668669, -0.00025523841094220916, -0.7487532505973302));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.9853848985862621, 0.0011442528907717002, -0.6002687283987052);
        let norm = Vec3::new(0.03653775353434474, 0.0028606322269292507, 0.9993281790032371);
        let t = 0.12167844661479818;
        let front_face = false;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);
    }

}

#[test]
pub fn world_hit_opt() {
    let world: Hittable = BvhNode::from(create_spheres());
    {
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(-1.7772153609911319, 0.00517260924540619, -1.0));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.9048564458630389, 0.0026335968731590856, -0.5091428229375708);
        let norm = Vec3::new(0.19028710827392215, 0.005267193746318171, 0.9817143541248583);
        let t = 0.5091428229375708;
        let front_face = true;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);

    }
    {
        let r = Ray::new(Vec3::new(-0.9048564458630389, 0.0026335968731590856, -0.5091428229375708), Vec3::new(-0.6627807911795999, -0.0005725727441804, -0.7488132577644417));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.98551439270837, 0.00256391690871263, -0.6002705996408596);
        let norm = Vec3::new(0.036214018229074885, 0.006409792271781576, 0.9993235008978509);
        let t = 0.1216962650679394;
        let front_face = false;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);
    }

    {
        let r = Ray::new(Vec3::new(-0.904730488748051, 0.0011753099041315778, -0.5091615959682413), Vec3::new(-0.6628487795668669, -0.00025523841094220916, -0.7487532505973302));
        let mut rec = HitRecord::new();
        assert_eq!(world.hit(r, 0.001, INFINITY, &mut rec), true);
        let vec = Vec3::new(-0.9853848985862621, 0.0011442528907717002, -0.6002687283987052);
        let norm = Vec3::new(0.03653775353434474, 0.0028606322269292507, 0.9993281790032371);
        let t = 0.12167844661479818;
        let front_face = false;
        assert_eq!(rec.p, vec);
        assert_eq!(rec.normal, norm);
        assert_eq!(rec.t, t);
        assert_eq!(rec.front_face, front_face);
    }

}