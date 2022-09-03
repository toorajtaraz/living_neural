use glium;
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
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }
    img.put_pixel(2, height - 1, Rgba([255, 255, 255, 255]));

    img
}

#[allow(dead_code)]
pub enum InitMode {
    RANDOM,
    CENTERTOP,
}

pub fn new_as_texture(
    init_type: InitMode,
    width: u32,
    height: u32,
    display: &glium::Display,
) -> glium::texture::SrgbTexture2d {
    let img = match init_type {
        InitMode::CENTERTOP => new_center_top(width, height),
        InitMode::RANDOM => new_random(width, height),
    };
    let img_dimensions = img.dimensions();
    let img_raw =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dimensions);
    glium::texture::SrgbTexture2d::new(display, img_raw).unwrap()
}

pub fn new_empty_texture(width: u32, height: u32, display: &glium::Display) -> glium::Texture2d {
    glium::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        width,
        height,
    )
    .unwrap()
}
