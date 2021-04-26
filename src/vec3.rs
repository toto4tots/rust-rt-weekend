
use crate::rtweekend::random_float_with_range;
use crate::rtweekend::random_float;
use std:: fmt;
use std::ops::{Add, Sub, Mul};
use std::convert::From;

pub fn close_enough(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-5
}

pub fn random() -> Vec3 {
    Vec3::new(random_float(), random_float(), random_float())
}

pub fn random_with_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(random_float_with_range(min, max), random_float_with_range(min, max), random_float_with_range(min, max))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_with_range(-1.0, 1.0);
        if p.mag_squared() >= 1.0 {
            continue
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        in_unit_sphere.scale(-1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new<N: Into<f64>>(x: N, y: N, z: N) -> Self {
        Vec3 {
            e: [x.into(), y.into(), z.into()]
        }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn scale(&self, val: f64) -> Vec3 {
        Vec3::new(self.e[0] * val, self.e[1] * val, self.e[2] *val)
    }

    pub fn magnitude(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn len(&self) -> usize {
        self.e.len()
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.e[0] * v.e[0] + 
        self.e[1] * v.e[1] + 
        self.e[2] * v.e[2] 
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0]
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.scale(1.0 / self.magnitude())
    }
    
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
       (self.e[0].abs() < s) &&(self.e[1].abs() < s) &&(self.e[2].abs() < s)
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - n.scale(self.dot(n) * 2.0)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(arr: [f64; 3]) -> Self {
        Vec3 {
            e: arr
        }
    }
}

impl<I: Into<Vec3>> Add<I> for Vec3 {
    type Output = Vec3;
    fn add(self, i: I) -> Vec3 {
        let other = i.into();
        Vec3::new(
            self.e[0] + other.e[0], 
            self.e[1] + other.e[1], 
            self.e[2] + other.e[2]
        )
    }
}

impl<I: Into<Vec3>> Sub<I> for Vec3 {
    type Output = Vec3;
    fn sub(self, i: I) -> Vec3 {
        let other = i.into();
        Vec3::new(
            self.e[0] - other.e[0], 
            self.e[1] - other.e[1], 
            self.e[2] - other.e[2]
        )
    }
}

impl<I: Into<Vec3>> Mul<I> for Vec3 {
    type Output = Vec3;
    fn mul(self, i: I) -> Vec3 {
        let other = i.into();
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2]
        )
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.e.len() {
            if !close_enough(self.e[i], other.e[i]) {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.e)
    }
}
