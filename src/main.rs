mod image;
mod ppm;

use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions};
use image::{Image, Pixel};

fn main() {
    println!("Trace all the rays!");
    let image = draw_gradient(400, 200);
    
    let ppm_str = ppm::to_ppm_p3_string(&image);
    write_to_file("output/render.ppm", &ppm_str).unwrap();
    println!("Rays have been traced!");
}

fn color_float_to_u8(color: f64) -> u8 {
    (color * 255.00) as u8
}

fn draw_gradient(width: u32, height: u32) -> Image {
    let mut image = Image::new(width, height);

    let height_float = height as f64;
    let width_float = width as f64;

    let blue = 0.2;
    let blue_i = color_float_to_u8(blue);

    for x in 0..height {
        for y in 0..width {
            let red = y as f64 / width_float;
            let green = (height - x - 1) as f64 / height_float;
            let red_i = color_float_to_u8(red);
            let green_i = color_float_to_u8(green);
            image.set(x, y, Pixel::new(red_i, green_i, blue_i))
        }
    }
    image
}

fn write_to_file(filepath: &str, data: &String) -> Result<(),io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath)?;
    
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}