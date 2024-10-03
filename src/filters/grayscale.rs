use image::{DynamicImage, RgbImage};

pub fn to_grayscale(img: &DynamicImage) -> RgbImage {
    let mut gray_img = RgbImage::new(img.width(), img.height());
    let rgb_img = img.to_rgb8();

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let gray_value = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        gray_img.put_pixel(x, y, image::Rgb([gray_value, gray_value, gray_value]));
    }

    gray_img
}