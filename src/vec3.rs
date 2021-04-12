
use std:: fmt;
use std::ops::{Add, Sub};

pub fn close_enough(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-5
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + 
    u.e[1] * v.e[1] + 
    u.e[2] * v.e[2] 
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0]
    )
}

#[derive(Debug)]
pub struct Vec3 {
    e: Vec<f64>
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            e: vec![x, y, z]
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
        let mut v = vec![];
        for i in 0..self.len() {
            v.push(self.e[i] * val);
        }
        Vec3::new(v[0], v[1], v[2])
    }

    pub fn multiply(&self, v: &Vec3) -> Vec3 {
        Vec3::new(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
    
    pub fn magnitude(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    fn mag_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn len(&self) -> usize {
        self.e.len()
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0], 
            self.e[1] + other.e[1], 
            self.e[2] + other.e[2]
        )
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0], 
            self.e[1] - other.e[1], 
            self.e[2] - other.e[2]
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




