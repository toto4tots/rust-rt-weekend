
use image::{RgbImage, Rgb};

use crate::{
    vec3,
    vec3::Vec3,
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

    pub fn set(&mut self, x: u32, y0: u32, color: Vec<f64>) {
        let flipped = true;
        let y = if flipped {self.height - 1 - y0} else {y0};
        if y < self.height {
            let r = (color[0] * 256.0) as u8;
            let g = (color[1] * 256.0) as u8;
            let b = (color[2] * 256.0) as u8;
            println!("xy = {} {} rgb = {}, {}, {} Color = {:?}", x, y, r, g, b, color);
            self.image.put_pixel(x, y, Rgb([r, g, b]));
        }

    }

    pub fn save(&self, name: &str) {
        self.image.save(name.to_owned()).unwrap();
    }
}
