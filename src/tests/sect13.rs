use crate::vec3::Color;
use crate::vec3;
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

pub fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();
    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    let s = Sphere::new(Point::new(0, -1000, 0), 1000.0, ground_material);
    world.add(s.into());

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
                    world.add(new_s.into());
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_with_range(0.5, 1.0);
                    let fuzz = random_float_with_range(0.0, 0.5);
                    sphere_material = Material::metal(albedo, fuzz);
                    let new_s = Sphere::new(center, 0.2, sphere_material);
                    world.add(new_s.into());
                } else {
                    // glass
                    sphere_material = Material::Dielectric(1.5);
                    let new_s = Sphere::new(center, 0.2, sphere_material);
                    world.add(new_s.into());
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    let s1 = Sphere::new(Point::new(0, 1, 0), 1.0, material1);
    world.add(s1.into());

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    let s2 = Sphere::new(Point::new(-4, 1, 0), 1.0, material2);
    world.add(s2.into());

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    let s3 = Sphere::new(Point::new(4, 1, 0), 1.0, material3);
    world.add(s3.into());

    world
}

#[test]
pub fn draw() {
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
    let world = random_scene();

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
    canvas.save("final.png");    



    


}
