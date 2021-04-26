
use crate::{
    vec3,
    vec3::Point,
    vec3::Color,
    vec3::Vec3,
    ray::Ray,
    ray::ray_color,
    hittable,
    hittable::Sphere,
    hittable::Hittable,
    hittable::HitRecord,
    hittable_list,
    hittable_list::HittableList,
    drawutils::Canvas
};

#[test]
pub fn hittable() {
    {
        let r = Ray::new(Point::new(0, 1, -5), Vec3::new(0, 0, 1));
        let s: Sphere = Default::default();
        let mut rec = hittable::HitRecord::new();
        assert_eq!(true, s.hit(r, 5.0, 5.0, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 2, -5), Vec3::new(0, 0, 1));
        let s: Sphere = Default::default();
        let mut rec = hittable::HitRecord::new();
        assert_eq!(false, s.hit(r, -100.0, 100.0, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 0), Vec3::new(0, 0, 1));
        let s: Sphere = Default::default();
        let mut rec = hittable::HitRecord::new();
        assert_eq!(true, s.hit(r, -1.0, 1.0, &mut rec));
        assert_eq!(false, s.hit(r, 1.5, 2.0, &mut rec));
        assert_eq!(false, s.hit(r, -2.0, -1.5, &mut rec));
    }
    {
        let r = Ray::new(Point::new(0, 0, 5), Vec3::new(0, 0, 1));
        let s: Sphere = Default::default();
        let mut rec = hittable::HitRecord::new();
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
        let s: Sphere = Default::default();
        let mut rec = HitRecord::new();
        assert_eq!(true, s.hit(r, 5.0, 5.0, &mut rec));
    }    
    {
        let s: Sphere = Default::default();
        let world = HittableList::new(vec![Box::new(s)]);

    }
}

// #[test]
// pub fn draw() {
//     // Image 
//     let aspect_ratio = 16.0 / 9.0;
//     let image_width = 400;
//     let fimage_width = 400f64;
//     let fimage_height = fimage_width / aspect_ratio;
//     let image_height = fimage_height as u32;
//     let samples_per_pixel = 1.0;

//     let mut canvas = Canvas::new(image_width, image_height);

//     // World
//     let mut s1 = Sphere::new(Point::new(0, 0, -1), 0.5);
//     let mut s2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
//     let world = HittableList::new(
//         vec![
//             Box::new(s1), 
//             Box::new(s2)
//         ]
//     );
    
//     // Camera
//     let viewport_height = 2.0;
//     let viewport_width = aspect_ratio * viewport_height;
//     let focal_length = 1.0;

//     let origin = Point::new(0.0, 0.0, 0.0);
//     let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
//     let vertical = Vec3::new(0.0, viewport_height, 0.0);
//     let lower_left_corner = ((origin - 
//                             horizontal.scale(1.0 / 2.0)) -
//                             vertical.scale(1.0 / 2.0)) - 
//                             Vec3::new(0.0, 0.0, focal_length);
    
//     for j in (0..image_height).rev() {
//         // println!("Scanlines remaining: {}", j);
//         for i in 0..image_width {
//             let u = (i as f64) / (fimage_width - 1.0);
//             let v = (j as f64) / (fimage_height - 1.0);
//             let r = Ray::new(origin, 
//                             lower_left_corner + 
//                             horizontal.scale(u) + 
//                             vertical.scale(v) - 
//                             origin);
//             let pixel_color = ray_color(r, &world);
//             canvas.set(i, j, pixel_color, samples_per_pixel);
//             // println!("{}", pixel_color);
//         }
//     }
//     canvas.save("ground.png");
// }
