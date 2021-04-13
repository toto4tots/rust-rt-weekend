
use image::{RgbImage, Rgb};

#[test]
pub fn draw() {
    let image_width = 256;
    let fimage_width = 256f64;
    let image_height = 256;
    let fimage_height = 256f64;
    let mut img = RgbImage::new(image_width, image_height);

    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = (i as f64) / (fimage_width - 1.0);
            let g = (j as f64) / (fimage_height - 1.0);
            let b = 0.25;

            let ur = (255.999 * r) as u8;
            let ug = (255.999 * g) as u8;
            let ub = (255.999 * b) as u8;

            if image_height - j < image_height && i < image_width {
                img.put_pixel(i, image_height - j, Rgb([ur, ug, ub,]));
            }   
        }
        // println!("Done");
    }
    img.save("image.png").unwrap();

}

