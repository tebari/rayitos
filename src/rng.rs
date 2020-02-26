use crate::vector::Vector3;
use rand::{thread_rng, Rng};

pub fn random_f64() -> f64 {
    thread_rng().gen_range(0.0, 1.0)
}

fn random_f64x3() -> [f64; 3] {
    [random_f64(), random_f64(), random_f64()]
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = 2.0 * Vector3::from_array(random_f64x3()) - Vector3::from_array([1.0; 3]);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
