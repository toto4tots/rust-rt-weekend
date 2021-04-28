use crate::{
    vec3::Point,
    vec3::Vec3,
    ray::Ray,
    rtweekend::degrees_to_radians,
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;        

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);


        Camera {
            origin: lookfrom,
            horizontal: u.scale(viewport_width),
            vertical: v.scale(viewport_height),
            lower_left_corner: 
                            lookfrom - 
                            (u.scale(viewport_width)).scale(1.0 / 2.0) - 
                            (v.scale(viewport_height)).scale(1.0 / 2.0) - 
                            w
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + 
            self.horizontal.scale(s) +
            self.vertical.scale(t) - 
            self.origin
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let lookfrom = Point::new(0, 0, 0);
        let lookat = Point::new(0, 0, -1);
        let vup = Vec3::new(0, 1, 0);
        let vfov = 90.0;
        let aspect_ratio = 16.0 / 9.0;
        Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio)
    }
}
