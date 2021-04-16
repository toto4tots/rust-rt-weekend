
use image::{RgbImage, Rgb};

use crate::{
    vec3,
    vec3::Vec3,
    rtweekend::clamp
};

pub struct Canvas {
    pub height: u32,
    pub width: u32,
    pub image: RgbImage
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let image = RgbImage::new(width, height);
        Canvas {
            width: width,
            height: height,
            image: image,
        }
    }

    pub fn get_color(&self, color: Vec3, samples_per_pixel: f64) -> [u8; 3] {
            let mut r = color.x();
            let mut g = color.y();
            let mut b = color.z();

            // Divide color by number of samples
            let scale = 1.0 / samples_per_pixel;
            r *= scale;
            g *= scale;
            b *= scale;

            // Write the translated [0, 255] value of each color component.
            [
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ]
    }

    pub fn set<I: Into<Vec3>>(&mut self, x: u32, y0: u32, color0: I, samples_per_pixel: f64) {
        let flipped = true;
        let y = if flipped {self.height - 1 - y0} else {y0};
        if y < self.height {
            let color = color0.into();
            let test = self.get_color(color, samples_per_pixel);
            // println!("{:?}", test);
            self.image.put_pixel(x, y, Rgb(self.get_color(color, samples_per_pixel)));
        }
    }

    pub fn save(&self, name: &str) {
        self.image.save(name.to_owned()).unwrap();
    }
}
