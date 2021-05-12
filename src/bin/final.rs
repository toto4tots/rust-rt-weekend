
extern crate rust_rt_weekend;

use std::env;

use rust_rt_weekend::bvh::BvhNode;
use std::{thread, time};
use rust_rt_weekend::vec3;
use rust_rt_weekend::vec3::Color;
use rust_rt_weekend::camera::Camera;
use rust_rt_weekend::ray::ray_color;
use rust_rt_weekend::ray::Ray;
use rust_rt_weekend::vec3::Vec3;
use rust_rt_weekend::hittable_list::HittableList;
use rust_rt_weekend::vec3::Point;
use rust_rt_weekend::hittable::Sphere;
use rust_rt_weekend::hittable::Hittable;
use rust_rt_weekend::drawutils::Canvas;
use rust_rt_weekend::material::Material;
use rust_rt_weekend::{
    rtweekend::random_float,
    rtweekend::random_float_with_range,
};
use std::f64::consts::PI;


pub fn random_scene_opt() -> Hittable {
    BvhNode::from(random_scene_vec())
}

pub fn random_scene_orig() -> Hittable {
    HittableList::new(random_scene_vec()).into()
}

pub fn random_scene_vec() -> Vec<Hittable> {
    let mut world: Vec<Hittable> = Default::default();
    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    let s = Sphere::new(Point::new(0, -1000, 0), 1000.0, ground_material);
    world.push(s.into());

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point::new(a as f64 + 0.9 * random_float(), 0.2, b as f64 + 0.9 * random_float());

            if (center - Point::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let mut sphere_material: Material = Default::default();
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::random() * vec3::random();
                    sphere_material = Material::Lambertian(albedo);
                    let new_s = Sphere::new(center, 0.2, sphere_material);
                    world.push(new_s.into());
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_with_range(0.5, 1.0);
                    let fuzz = random_float_with_range(0.0, 0.5);
                    sphere_material = Material::metal(albedo, fuzz);
                    let new_s = Sphere::new(center, 0.2, sphere_material);
                    world.push(new_s.into());
                } else {
                    // glass
                    sphere_material = Material::Dielectric(1.5);
                    let new_s = Sphere::new(center, 0.2, sphere_material);
                    world.push(new_s.into());
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    let s1 = Sphere::new(Point::new(0, 1, 0), 1.0, material1);
    world.push(s1.into());

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    let s2 = Sphere::new(Point::new(-4, 1, 0), 1.0, material2);
    world.push(s2.into());

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    let s3 = Sphere::new(Point::new(4, 1, 0), 1.0, material3);
    world.push(s3.into());

    world
}

pub fn main() {
    // Image 
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let fimage_width = 1200f64;
    let fimage_height = fimage_width / aspect_ratio;
    let image_height = fimage_height as u32;
    let samples_per_pixel = 500.0;
    let max_depth = 50;

    let mut canvas = Canvas::new(image_width, image_height);

    // World
    let args: Vec<String> = env::args().collect();
    let arg: Option<&str> = args.get(1).map(|s| s.as_ref());
    let world: Hittable = match arg {
        None | Some("opt") => {
            println!("Using opt");
            random_scene_opt()
        }
        Some("old") => {
            println!("Using old");
            random_scene_orig()
        }
        Some(s) => panic!("Unrecognized option {}", s),
    };


    // Camera
    let lookfrom = Point::new(13, 2, 3);
    let lookat = Point::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio
    );

    let start = time::Instant::now();
    // Render 
    for j in (0..image_height).rev() {
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
        let now = time::Instant::now();
        println!("Scanlines remaining: {}, {:?}", j, now - start);
    }
    canvas.save("finalTest.png");    
}

