
use image::{RgbImage, Rgb};
use crate::{
    drawutils
};

#[test]
pub fn draw() {
    let image_width = 256;
    let fimage_width = 256f64;
    let image_height = 256;
    let fimage_height = 256f64;
    let mut img = RgbImage::new(image_width, image_height);
    let mut canvas = drawutils::Canvas::new(image_width, image_height);

    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = (i as f64) / (fimage_width - 1.0);
            let g = (j as f64) / (fimage_height - 1.0);
            let b = 0.25;

            let ur = (255.999 * r) as u8;
            let ug = (255.999 * g) as u8;
            let ub = (255.999 * b) as u8;

            canvas.set(i, j, [r, g, b]);
        }
    }
    canvas.save("image2.png");

}

