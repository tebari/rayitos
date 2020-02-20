use super::ray::Ray;
use super::image::{Image, Pixel, color_float_to_u8};
use na::{Point3, Vector3};

pub fn draw_blank(width: u32, height: u32) -> Image {
    Image::new(width, height)
}

pub fn draw_gradient(width: u32, height: u32) -> Image {
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

fn hit_sphere(center: Point3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}

pub fn pixel_from_ray(ray: &Ray) -> Pixel {
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n = ray.point_at(t) - Vector3::new(0.0,0.0,-1.0);
        let n_normal = n.coords.normalize();
        return Pixel::from(0.5 * Vector3::new(n_normal[0]+1.0, n_normal[1]+1.0, n_normal[2]+1.0));
    }
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    let color_vector = (1.0 - t) * Vector3::new(1.0,1.0,1.0) + t*Vector3::new(0.5,0.7,1.0);
    Pixel::from(color_vector)
}

pub fn draw_sky(width: u32, height: u32) -> Image {
    let mut image = Image::new(width, height);

    let origin = Point3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    let height_float = height as f64;
    let width_float = width as f64;

    for x in 0..height {
        for y in 0..width {
            let u = y as f64 / width_float;
            let v = (height - x - 1) as f64 / height_float;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            image.set(x, y, pixel_from_ray(&r));
        }
    }
    
    image
}
