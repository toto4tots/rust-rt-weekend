use crate::vec3::Color;
use crate::camera::Camera;
use crate::ray::ray_color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::vec3::Point;
use crate::hittable::Sphere;
use crate::drawutils::Canvas;
use crate::{
    rtweekend::random_float,
    rtweekend::random_float_with_range,
};


// #[test]
// pub fn diffusion() {
//     // Image 
//     let aspect_ratio = 16.0 / 9.0;
//     let image_width = 400;
//     let fimage_width = 400f64;
//     let fimage_height = fimage_width / aspect_ratio;
//     let image_height = fimage_height as u32;
//     let samples_per_pixel = 100.0;
//     let max_depth = 50;

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
//     let cam = Camera::new();

//     // Render 
//     for j in (0..image_height).rev() {
//         println!("Scanlines remaining: {}", j);
//         for i in 0..image_width {
//             let mut pixel_color = Color::new(0, 0, 0);
//             for s in 0..samples_per_pixel as usize {
//                 let u = (i as f64 + random_float()) / (fimage_width - 1.0);
//                 let v = (j as f64 + random_float()) / (fimage_height - 1.0);
//                 let r = cam.get_ray(u, v);
//                 pixel_color = ray_color(r, &world, max_depth) + pixel_color;
//             }
//             canvas.set(i, j, pixel_color, samples_per_pixel);

//         }
//     }
//     canvas.save("diffusion.png");
// }
