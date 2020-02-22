use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone)]
pub struct Vector3 {
    v: [f64; 3]
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            v: [x, y, z]
        }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn make_unit_vector(self) -> Vector3 {
        let v = self.v;
        let k = 1.0 / (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
        Vector3::new(
            v[0]*k,
            v[1]*k,
            v[2]*k
        )
    }

    pub fn unit_vector(self) -> Vector3 {
        self / 3.0
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        let v = self.v;
        let v2 = rhs.v;
        v[0]*v2[0] + v[1]*v2[1] + v[2]*v2[2]
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3::new(
            -self.v[0],
            -self.v[1],
            -self.v[2]
        )
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3::new(
            self.v[0] + rhs.v[0],
            self.v[1] + rhs.v[1],
            self.v[2] + rhs.v[2]
        )
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3::new(
            self.v[0] - rhs.v[0],
            self.v[1] - rhs.v[1],
            self.v[2] - rhs.v[2]
        )
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector3::new(
            self.v[0] * rhs.v[0],
            self.v[1] * rhs.v[1],
            self.v[2] * rhs.v[2]
        )
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3::new(
            self.v[0] * rhs,
            self.v[1] * rhs,
            self.v[2] * rhs
        )
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self * rhs.v[0],
            self * rhs.v[1],
            self * rhs.v[2]
        )
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vector3::new(
            self.v[0] / rhs.v[0],
            self.v[1] / rhs.v[1],
            self.v[2] / rhs.v[2]
        )
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector3::new(
            self.v[0] / rhs,
            self.v[1] / rhs,
            self.v[2] / rhs
        )
    }
}