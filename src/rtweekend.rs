
use rand::Rng;

pub fn random_float() -> f64 {
    // return float [0, 1)
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_float_with_range(min: f64, max: f64) -> f64 {
    // return random real in [min, max)
    min + (max - min) * random_float()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}
