use super::ray::Ray;
use super::image::{Image, Pixel, color_float_to_u8};
use super::vector::Vector3;
use super::hittables::{Hittable, HittableList, Sphere};

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

struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    fn default() -> Camera {
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0)
        }
    }

    fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical)
    }
}

pub fn color(ray: &Ray, world: &dyn Hittable) -> Pixel {
    let hit_record = world.hit(ray, 0.0, std::f64::MAX);
    match hit_record {
        Some(rec) => {
            Pixel::from(0.5 * Vector3::new(rec.normal.x()+1.0, rec.normal.y()+1.0, rec.normal.z()+1.0))
        },
        None => {
            let unit_direction = ray.direction().make_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            let color_vector = (1.0 - t) * Vector3::new(1.0,1.0,1.0) + t*Vector3::new(0.5,0.7,1.0);
            Pixel::from(color_vector)
        }
    }
}

pub fn draw_sky(width: u32, height: u32) -> Image {
    let height_float = height as f64;
    let width_float = width as f64;

    let mut image = Image::new(width, height);
    let camera = Camera::default();

    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    for x in 0..height {
        for y in 0..width {
            let u = y as f64 / width_float;
            let v = (height - x - 1) as f64 / height_float;
            let r = camera.ray(u, v);
            image.set(x, y, color(&r, &world));
        }
    }
    
    image
}
