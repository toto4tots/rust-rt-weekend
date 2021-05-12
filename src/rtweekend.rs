
use std::f64::consts::PI;
use rand::Rng;

pub fn random_int_with_range(a: i32, b: i32) -> i32 {
    random_float_with_range(a as f64, b as f64) as i32
}

pub fn random_float() -> f64 {
    // return float [0, 1)
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_float_with_range(min: f64, max: f64) -> f64 {
    // return random real in [min, max)
    min + (max - min) * random_float()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}
