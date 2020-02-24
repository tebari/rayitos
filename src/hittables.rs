use crate::vector::Vector3;
use crate::ray::Ray;
use crate::rng;
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: &'a Box<dyn Material>
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (&Vector3, Ray);
}

pub struct Lambertian {
    albedo: Vector3
}

impl Lambertian {
    pub fn from(albedo: Vector3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (&Vector3, Ray) {
        let target = hit_record.p + hit_record.normal + rng::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        let attenuation = &self.albedo;
        (&attenuation, scattered)
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center, radius, material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            let point = ray.point_at(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            let point = ray.point_at(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material
                });
            }
        }
        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList {
            list
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record : Option<HitRecord> = None;
        let mut closet_so_far = t_max;
        for hittable in &self.list {
            let new_hit = hittable.hit(ray, t_min, closet_so_far);
            if new_hit.is_some() {
                let hit = new_hit.unwrap();
                closet_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}