mod image;
mod ppm;
mod renderers;

use std::env;
use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions};

fn main() {
    println!("Trace all the rays!");
    let args: Vec<String> = env::args().collect();
    let renderer = &args[1];

    let width = 1600;
    let height = 800;

    let image = match renderer.as_ref() {
        "gradient" => renderers::draw_gradient(width, height),
        _ => renderers::draw_blank(width, height)
    };
    
    println!("Generating Output File");
    let ppm_str = ppm::to_ppm_p3_string(&image);
    println!("Write file");
    write_to_file("output/render.ppm", &ppm_str).unwrap();
    println!("Rays have been traced!");
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