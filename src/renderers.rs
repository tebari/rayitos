use crate::ray::Ray;
use crate::image::{Image, Pixel, color_float_to_u8};
use crate::vector::Vector3;
use crate::hittables::{Hittable, HittableList, Sphere, Lambertian, Metal, Dielectric};
use crate::rng::{random_f64, random_in_unit_sphere};

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
    u: Vector3,
    v: Vector3,
    lens_radius: f64
}

impl Camera {
    fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect: f64,
           aperture: f64, focus_dist: f64) -> Camera {

        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);
        let lower_left_corner = lookfrom
            - half_width * focus_dist * u
            - half_height * focus_dist * v
            - focus_dist * w;

        Camera {
            lower_left_corner,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            u, v, lens_radius
        }
    }

    fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset,
            self.lower_left_corner + s*self.horizontal
            + t*self.vertical - self.origin - offset
        )
    }
}

fn color(ray: &Ray, world: &dyn Hittable, depth: u8) -> Vector3 {
    let hit_record = world.hit(ray, 0.001, std::f64::MAX);
    match hit_record {
        Some(rec) => {
            let (attenuation, scattered, scatter) = rec.material.scatter(ray, &rec);
            if scatter && depth < 50 {
                return attenuation * color(&scattered, world, depth+1);
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

pub fn trio_sphere_scene() -> HittableList {
    HittableList::new(vec![
        Box::new(Sphere::new(
            Vector3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::from(Vector3::new(0.1, 0.2, 0.5)))
        )),
        Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::from(Vector3::new(0.8, 0.8, 0.0)))
        )),
        Box::new(Sphere::new(
            Vector3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3))
        )),
        Box::new(Sphere::new(
            Vector3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Dielectric::new(1.5))
        )),
        Box::new(Sphere::new(
            Vector3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Dielectric::new(1.5))
        )),
    ])
}

fn render(width: u32, height: u32, camera: Camera, world: HittableList) -> Image {
    let mut image = Image::new(width, height);
    let aa_samples = 100;
    let aa_samples_f = aa_samples as f64;
    let height_float = height as f64;
    let width_float = width as f64;

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

pub fn draw_trio(width: u32, height: u32) -> Image {
    let height_float = height as f64;
    let width_float = width as f64;

    let lookfrom = Vector3::new(3.0, 3.0, 2.0);
    let lookat = Vector3::new(0.0, 0.0, -1.0);
    let distance_to_focus = (lookfrom-lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        width_float / height_float,
        aperture,
        distance_to_focus
    );
    render(width, height, camera, trio_sphere_scene())
}
