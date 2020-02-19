use super::image;

fn header(width: u32, height: u32) -> String {
    format!("P3 {} {} 255", height.to_string(), width.to_string())
}

fn pixelmap_to_string(image: &image::Image) -> String {
    let string_size = (image.get_pixel_count() * 4 + 1) as usize;
    let mut ppm_string = String::with_capacity(string_size);
    for x in 0..image.get_height() {
        for y in 0..image.get_width() {
            let pixel = image.get(x, y);
            ppm_string.push_str(&pixel.get_red().to_string());
            ppm_string.push_str(" ");
            ppm_string.push_str(&pixel.get_green().to_string());
            ppm_string.push_str(" ");
            ppm_string.push_str(&pixel.get_blue().to_string());
            ppm_string.push_str("\n");
        }
    }
    ppm_string
}

pub fn to_ppm_p3_string(image: &image::Image) -> String {
    format!("{}\n{}",
        header(image.get_height(), image.get_width()),
        pixelmap_to_string(image)
    )
}
