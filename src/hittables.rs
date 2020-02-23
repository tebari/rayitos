use crate::vector::Vector3;
use crate::ray::Ray;
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3,
    pub normal: Vector3
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere {
            center, radius
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
                    normal: (point - self.center) / self.radius
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            let point = ray.point_at(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius
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