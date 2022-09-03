use image::{Rgba, RgbaImage};
use rand::prelude::*;

pub fn new_random(width: u32, height: u32) -> RgbaImage {
    let mut rng = rand::thread_rng();
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let t = rng.gen::<u8>();
            img.put_pixel(x, y, Rgba([t, t, t, t]));
        }
    }

    img
}

pub fn new_center_top(width: u32, height: u32) -> RgbaImage {
    let mut rng = rand::thread_rng();
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }
    img.put_pixel(2, height - 1, Rgba([255, 255, 255, 255]));

    img
}
