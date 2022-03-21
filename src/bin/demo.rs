extern crate rust_rt_weekend;

use rust_rt_weekend::camera::Camera;
use rust_rt_weekend::drawutils::Canvas;
use rust_rt_weekend::hittable::Sphere;
use rust_rt_weekend::hittable_list::HittableList;
use rust_rt_weekend::material::Material;
use rust_rt_weekend::ray::ray_color;
use rust_rt_weekend::ray::Ray;
use rust_rt_weekend::vec3::Color;
use rust_rt_weekend::vec3::Point;
use rust_rt_weekend::vec3::Vec3;
use rust_rt_weekend::{rtweekend::random_float, rtweekend::random_float_with_range};
use std::f64::consts::PI;
use std::{thread, time};

pub fn rotate(x: f64, z: f64, rad: f64) -> [f64; 2] {
  let cos = rad.cos();
  let sin = rad.sin();
  let nx = (cos * (x - 0.0)) - (sin * (z + 1.0)) + 0.0; // because center 0, 0, -1
  let nz = (cos * (z + 1.0)) + (sin * (x - 0.0)) - 1.0;
  [nx, nz]
}

pub fn getRotationCamera(camera_pos: [f64; 2], aspect_ratio: f64) -> Camera {
  Camera::new(
    Point::new(camera_pos[0], 0.0, camera_pos[1]),
    Point::new(0, 0, -1),
    Vec3::new(0, 1, 0),
    90.0,
    aspect_ratio,
  )
}

pub fn frame(camera_pos: [f64; 2], frame_num: f64, f: &dyn Fn([f64; 2], f64) -> Camera) {
  // Image
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 400;
  let fimage_width = 400f64;
  let fimage_height = fimage_width / aspect_ratio;
  let image_height = fimage_height as u32;
  let samples_per_pixel = 100.0;
  let max_depth = 50;

  let mut canvas = Canvas::new(image_width, image_height);

  // World
  let material_ground = Material::Lambertian(Color::new(0.2, 0.8, 0.2));
  let material_center = Material::Dielectric(1.5);
  let material_left = Material::Metal(Color::new(0.6, 0.3, 0.3), 0.0);
  let material_right1 = Material::Lambertian(Color::new(0.2, 0.6, 0.6));
  let material_right2 = Material::Lambertian(Color::new(0.9, 0.2, 0.2));

  let s1 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, material_ground);
  let s2 = Sphere::new(Point::new(1.0, -0.25, -0.5), 0.25, material_right1);
  let s3 = Sphere::new(Point::new(0.8, 0.0, -1.5), 0.5, material_right2);
  let s4 = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left);
  let s5 = Sphere::new(Point::new(0.0, -0.25, -1.0), 0.25, material_center);

  let world = HittableList::new(vec![s1.into(), s2.into(), s3.into(), s4.into(), s5.into()]).into();

  // Camera
  let cam: Camera = f(camera_pos, aspect_ratio);

  // Render
  for j in (0..image_height).rev() {
    println!("Scanlines remaining: {}", j);
    for i in 0..image_width {
      let mut pixel_color = Color::new(0, 0, 0);
      for s in 0..samples_per_pixel as usize {
        let u = (i as f64 + random_float()) / (fimage_width - 1.0);
        let v = (j as f64 + random_float()) / (fimage_height - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color = ray_color(r, &world, max_depth) + pixel_color;
      }
      canvas.set(i, j, pixel_color, samples_per_pixel);
    }
  }
  canvas.save(&format!("demotest/rotation/frame{}.png", frame_num).to_string());
}

pub fn main() {
  let mut x = 0.0;
  let mut z = 1.0;
  let start = time::Instant::now();

  for i in 0..1 {
    let pair = rotate(x, z, (PI / 24.0) * i as f64);
    frame(pair, i as f64, &getRotationCamera);
    let now = time::Instant::now();
    println!("Frame {}, Time {:?}", i, now - start);
  }
}
