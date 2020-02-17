mod image;
mod ppm;

use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions};
use image::{Image, Pixel};

fn main() {
    let mut img = Image::new(3, 3);
    img.set(0,0, Pixel::new(255, 0, 0));
    img.set(0,1, Pixel::new(0, 255, 0));
    img.set(0,2, Pixel::new(0, 0, 255));
    img.set(1,0, Pixel::new(255, 255, 255));
    img.set(2,2, Pixel::new(255, 255, 255));
    println!("Trace all the rays!");
    
    let ppm_str = ppm::to_ppm_p3_string(&img);
    write_to_file("output/render.ppm", &ppm_str).unwrap();
    print!("{}", ppm_str);
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