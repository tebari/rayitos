use super::image;

fn header(width: u32, height: u32) -> String {
    format!("P3 {} {} 255", width.to_string(), height.to_string())
}

fn pixelmap_to_string(image: &image::Image) -> String {
    let string_size = (image.get_pixel_count() * 4 + 1) as usize;
    let mut ppm_string = String::with_capacity(string_size);
    for x in 0..image.get_height() {
        for y in 0..image.get_width() {
            let pixel = image.get(x, y);
            if x == 0 && y == 0{
                ppm_string = format!("{} {} {}\n",
                    pixel.get_red(), pixel.get_green(), pixel.get_blue());
            } else {
                ppm_string = format!("{} {} {} {}\n",
                    ppm_string, pixel.get_red(), pixel.get_green(), pixel.get_blue());
            }
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
