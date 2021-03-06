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
use crate::{
    material,
    material::Material
};

// #[test]
// pub fn draw() {
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
//     let material_ground = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
//     let material_center = Material::Lambertian(Color::new(0.7, 0.3, 0.3));
//     let material_left = Material::Metal(Color::new(0.8, 0.8, 0.8), 0.0);
//     let material_right = Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0);

//     let s1 = Sphere::new(
//         Point::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground
//     );
//     let s2 = Sphere::new(
//         Point::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center
//     );
//     let s3 = Sphere::new(
//         Point::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left
//     );
//     let s4 = Sphere::new(
//         Point::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right
//     );

//     let world = HittableList::new(
//         vec![
//             Box::new(s1), 
//             Box::new(s2),
//             Box::new(s3),
//             Box::new(s4),
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
//     canvas.save("metal.png");
// }

// #[test]
// pub fn fuzzy() {
//     // Image 
//     let aspect_ratio = 16.0 / 9.0;
//     let image_width = 400;
//     let fimage_width = 400f64;
//     let fimage_height = fimage_width / aspect_ratio;
//     let image_height = fimage_height as u32;
//     let samples_per_pixel = 100.0;
//     let max_depth = 50;

//     let mut canvas = Canvas::new(image_width, image_height);
// //  material: Material::Lambertian(Color::new(0, 0, 0)),
//     // World
//     let material_ground = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
//     let material_center = Material::Lambertian(Color::new(0.7, 0.3, 0.3));
//     let material_left = Material::metal(Color::new(0.8, 0.8, 0.8), 0.3);
//     let material_right = Material::metal(Color::new(0.8, 0.6, 0.2), 1.0);

//     let s1 = Sphere::new(
//         Point::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground
//     );
//     let s2 = Sphere::new(
//         Point::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center
//     );
//     let s3 = Sphere::new(
//         Point::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left
//     );
//     let s4 = Sphere::new(
//         Point::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right
//     );

//     let world = HittableList::new(
//         vec![
//             Box::new(s1), 
//             Box::new(s2),
//             Box::new(s3),
//             Box::new(s4),
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
//     canvas.save("fuzzy.png");    

// }