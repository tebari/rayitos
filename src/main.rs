use rayitolib::{ppm, renderers};
use std::env;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Trace all the rays!");
    let args: Vec<String> = env::args().collect();
    let renderer = &args[1];
    let width_str = args.get(2);
    let height_str = args.get(3);

    let width = get_uint_or(width_str, 200);
    let height = get_uint_or(height_str, 100);

    let image = match renderer.as_ref() {
        "gradient" => renderers::draw_gradient(width, height),
        "trio" => renderers::draw_trio(width, height),
        "random" => renderers::draw_random(width, height),
        _ => renderers::draw_blank(width, height),
    };

    println!("Generating Output File");
    let ppm_str = ppm::to_ppm_p3_string(&image);
    println!("Write file");
    write_to_file("output/render.ppm", &ppm_str).unwrap();
    println!("Rays have been traced!");
}

fn get_uint_or(number: Option<&String>, default: u32) -> u32 {
    let parsed_num = number.map(|num_str| num_str.parse::<u32>());
    match parsed_num {
        Some(Ok(num)) => num,
        _ => default,
    }
}

fn write_to_file(filepath: &str, data: &String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath)?;

    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}
