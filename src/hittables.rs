use crate::ray::Ray;
use crate::rng;
use crate::vector::Vector3;
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: &'a Box<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vector3, Ray, bool);
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn from(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (Vector3, Ray, bool) {
        let target = hit_record.p + hit_record.normal + rng::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        let attenuation = self.albedo;
        (attenuation, scattered, true)
    }
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };

        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vector3, Ray, bool) {
        let reflected = reflect(ray_in.direction().unit_vector(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * rng::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        let scatter = scattered.direction().dot(hit_record.normal) > 0.0;
        (attenuation, scattered, scatter)
    }
}

fn refract(v: Vector3, n: Vector3, ni_over_nt: f64) -> Option<Vector3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted);
    }
    None
}

fn shlick(cosine: f64, reflective_index: f64) -> f64 {
    let r0 = (1.0 - reflective_index) / (1.0 + reflective_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dielectric {
    reflective_index: f64,
}

impl Dielectric {
    pub fn new(reflective_index: f64) -> Dielectric {
        Dielectric { reflective_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vector3, Ray, bool) {
        let reflected = reflect(ray_in.direction(), hit_record.normal);
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let positive_direction = ray_in.direction().dot(hit_record.normal) > 0.0;

        let (outward_normal, ni_over_nt, cosine) = if positive_direction {
            let cosine = self.reflective_index * ray_in.direction().dot(hit_record.normal)
                / ray_in.direction().length();
            (-hit_record.normal, self.reflective_index, cosine)
        } else {
            let cosine =
                (-(ray_in.direction().dot(hit_record.normal))) / ray_in.direction().length();
            (hit_record.normal, 1.0 / self.reflective_index, cosine)
        };

        let refracted_opt = refract(ray_in.direction(), outward_normal, ni_over_nt);
        let new_ray = match refracted_opt {
            Some(refracted) => {
                let reflected_prob = shlick(cosine, self.reflective_index);
                if rng::random_f64() < reflected_prob {
                    Ray::new(hit_record.p, reflected)
                } else {
                    Ray::new(hit_record.p, refracted)
                }
            }
            _ => Ray::new(hit_record.p, reflected),
        };

        (attenuation, new_ray, true)
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            let point = ray.point_at(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            let point = ray.point_at(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material,
                });
            }
        }
        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: vec![] }
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, hittable: T) {
        self.list.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
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
