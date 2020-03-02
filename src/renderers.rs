use crate::hittables::{Dielectric, Hittable, HittableList, Lambertian, Metal, Sphere};
use crate::image::{color_float_to_u8, Image, Pixel, Tile};
use crate::ray::Ray;
use crate::rng::{random_f64, random_in_unit_sphere};
use crate::vector::Vector3;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::thread;

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
    lens_radius: f64,
}

impl Camera {
    fn new(
        lookfrom: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);
        let lower_left_corner =
            lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;

        Camera {
            lower_left_corner,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            u,
            v,
            lens_radius,
        }
    }

    fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn color(ray: &Ray, world: &dyn Hittable, depth: u8) -> Vector3 {
    let hit_record = world.hit(ray, 0.001, std::f64::MAX);
    match hit_record {
        Some(rec) => {
            let (attenuation, scattered, scatter) = rec.material.scatter(ray, &rec);
            if scatter && depth < 50 {
                return attenuation * color(&scattered, world, depth + 1);
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }
        None => {
            let unit_direction = ray.direction().make_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    world.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::from(Vector3::new(0.5, 0.5, 0.5))),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vector3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::from(Vector3::new(
                            random_f64() * random_f64(),
                            random_f64() * random_f64(),
                            random_f64() * random_f64(),
                        ))),
                    ));
                } else if choose_mat < 0.95 {
                    //metal
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + random_f64()),
                                0.5 * (1.0 + random_f64()),
                                0.5 * (1.0 + random_f64()),
                            ),
                            0.5 * random_f64(),
                        )),
                    ));
                } else {
                    //glass
                    world.add(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::from(Vector3::new(0.4, 0.2, 0.1))),
    ));
    world.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}

pub fn trio_sphere_scene() -> HittableList {
    let mut world = HittableList::new();
    world.add(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::from(Vector3::new(0.1, 0.2, 0.5))),
    ));
    world.add(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::from(Vector3::new(0.8, 0.8, 0.0))),
    ));
    world.add(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3)),
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    ));
    world
}

fn render_lines(width: u32, height: u32, camera: &Camera, world: &HittableList, tile: &mut Tile) {
    let aa_samples = 100;
    let aa_samples_f = aa_samples as f64;
    let end_x = tile.start_x() + tile.image().get_height(); //offsetted width
    let end_y = tile.start_y() + tile.image().get_width(); //offsetted height

    for x in tile.start_x()..end_x {
        for y in tile.start_y()..end_y {
            let mut color_vector = Vector3::new(0.0, 0.0, 0.0);
            let l = (height - x - 1) as f64;
            let c = y as f64;
            for _s in 0..aa_samples {
                let u = (c + random_f64()) / width as f64;
                let v = (l + random_f64()) / height as f64;
                let r = camera.ray(u, v);
                color_vector += color(&r, world, 0);
            }

            let color_vector_aa = color_vector / aa_samples_f;
            let color_vector_gamma = color_vector_aa.square_root();
            tile.set(x, y, Pixel::from(color_vector_gamma));
        }
    }
}

fn multithread_render(
    width: u32,
    height: u32,
    camera: Camera,
    world: HittableList,
    tiles: Vec<Tile>,
) -> Vec<Tile> {
    let num_cpus = num_cpus::get();
    let tiles_iter = Arc::new(Mutex::new(tiles.into_iter()));
    let rendered_tiles = Arc::new(Mutex::new(vec![]));
    let camera = Arc::new(camera);
    let world = Arc::new(world);

    let mut handles = vec![];
    for i in 0..num_cpus {
        let tiles_iter = Arc::clone(&tiles_iter);
        let rendered_tiles = Arc::clone(&rendered_tiles);
        let camera = Arc::clone(&camera);
        let world = Arc::clone(&world);
        let handle = thread::Builder::new()
            .name(format!("rayito-worker-{}", i))
            .spawn(move || loop {
                let tile = {
                    let mut iter = tiles_iter.lock().unwrap();
                    iter.next()
                };
                match tile {
                    Some(mut tile) => {
                        render_lines(width, height, &camera, &world, &mut tile);
                        rendered_tiles.lock().unwrap().push(tile);
                    }
                    _ => break,
                }
            })
            .unwrap();
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let tiles = Arc::try_unwrap(rendered_tiles);
    if let Ok(tiles) = tiles {
        tiles.into_inner().unwrap()
    } else {
        panic!("Tiles still held by worker threads");
    }
}

fn render(width: u32, height: u32, camera: Camera, world: HittableList) -> Image {
    let jobs = 32;

    let lines_per_tile = height / jobs;
    let tile_count = if height % jobs == 0 { jobs } else { jobs + 1 };

    let mut tiles: Vec<Tile> = Vec::with_capacity(tile_count as usize);
    for i in 0..tile_count {
        let start_x = i * lines_per_tile;
        let tile_height = if i < tile_count - 1 {
            lines_per_tile
        } else {
            height - lines_per_tile * i
        };
        tiles.push(Tile::new(start_x, 0, width, tile_height));
    }

    // tiles
    //     .iter_mut()
    //     .for_each(|tile| render_lines(width, height, &camera, &world, tile));

    let tiles = multithread_render(width, height, camera, world, tiles);
    Image::from_tiles(width, height, tiles)
}

pub fn draw_trio(width: u32, height: u32) -> Image {
    let height_float = height as f64;
    let width_float = width as f64;

    let lookfrom = Vector3::new(3.0, 3.0, 2.0);
    let lookat = Vector3::new(0.0, 0.0, -1.0);
    let distance_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        width_float / height_float,
        aperture,
        distance_to_focus,
    );
    render(width, height, camera, trio_sphere_scene())
}

pub fn draw_random(width: u32, height: u32) -> Image {
    let height_float = height as f64;
    let width_float = width as f64;

    let lookfrom = Vector3::new(12.0, 1.5, 3.0);
    let lookat = Vector3::new(0.0, 1.0, 0.0);
    let distance_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        width_float / height_float,
        aperture,
        distance_to_focus,
    );
    render(width, height, camera, random_scene())
}
