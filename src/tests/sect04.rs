
use crate::{
    vec3,
    vec3::Vec3,
    vec3::Point,
    ray,
    ray::Ray,
    ray::ray_color,
    drawutils::Canvas
};

#[test]
pub fn create_ray() {
    let p = Point::new(2, 3, 4);
    let v = Vec3::new(1, 0, 0);
    let r = Ray::new(p, v);
    assert_eq!(r.origin, p);
    assert_eq!(r.direction, v);
    let pos1 = r.at(0);
    assert_eq!(pos1, p);

    let ans2 = Vec3::new(3, 3, 4);
    assert_eq!(r.at(1), ans2);

    let ans3 = Vec3::new(1, 3, 4);
    assert_eq!(r.at(-1), ans3);

    let ans4 = Vec3::new(4.5, 3.0, 4.0);
    assert_eq!(r.at(2.5), ans4);
}

#[test]
pub fn draw() {
    // Image 
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let fimage_width = 400f64;
    let fimage_height = fimage_width / aspect_ratio;
    let image_height = fimage_height as u32;

    let mut canvas = Canvas::new(image_width, image_height);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = ((origin - 
                            horizontal.scale(1.0 / 2.0)) -
                            vertical.scale(1.0 / 2.0)) - 
                            Vec3::new(0.0, 0.0, focal_length);
    
    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / (fimage_width - 1.0);
            let v = (j as f64) / (fimage_height - 1.0);
            let r = Ray::new(origin, 
                            lower_left_corner + 
                            horizontal.scale(u) + 
                            vertical.scale(v) - 
                            origin);
            let pixel_color = ray_color(&r);
            canvas.set(i, j, pixel_color);
        }
    }
    canvas.save("colorsphere.png");
}
