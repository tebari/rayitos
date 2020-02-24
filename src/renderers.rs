use crate::ray::Ray;
use crate::image::{Image, Pixel, color_float_to_u8};
use crate::vector::Vector3;
use crate::hittables::{Hittable, HittableList, Sphere, Lambertian};
use crate::rng::random_f64;

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

fn color(ray: &Ray, world: &dyn Hittable, depth: u8) -> Vector3 {
    let hit_record = world.hit(ray, 0.001, std::f64::MAX);
    match hit_record {
        Some(rec) => {
            let (attenuation, scattered) = rec.material.scatter(ray, &rec);
            if depth < 50 {
                return (*attenuation) * color(&scattered, world, depth+1);
            } else {
                return Vector3::new(0.0,0.0,0.0)
            }
        },
        None => {
            let unit_direction = ray.direction().make_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vector3::new(1.0,1.0,1.0) + t*Vector3::new(0.5,0.7,1.0)
        }
    }
}

pub fn draw_sky(width: u32, height: u32) -> Image {
    let aa_samples = 100;
    let aa_samples_f = aa_samples as f64;
    let height_float = height as f64;
    let width_float = width as f64;

    let mut image = Image::new(width, height);
    let camera = Camera::default();

    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Vector3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::from(Vector3::new(0.8, 0.3, 0.3)))
        )),
        Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::from(Vector3::new(0.8, 0.8, 0.0)))
        )),
    ]);

    for x in 0..height {
        for y in 0..width {
            let mut color_vector = Vector3::new(0.0, 0.0, 0.0);
            let l = (height - x - 1) as f64;
            let c = y as f64;
            for _s in 0..aa_samples {
                let u = (c + random_f64()) / width_float;
                let v = (l + random_f64()) / height_float;
                let r = camera.ray(u, v);
                color_vector += color(&r, &world, 0);
            }
            
            let color_vector_aa = color_vector / aa_samples_f;
            let color_vector_gamma = color_vector_aa.square_root();
            image.set(x, y, Pixel::from(color_vector_gamma));
        }
    }
    
    image
}
