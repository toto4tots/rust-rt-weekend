
mod tests {
    pub mod sect02;
    pub mod sect03;
    pub mod sect04;
    pub mod sect06;
    pub mod sect07;
    pub mod sect08;
    pub mod sect09;
}
pub mod vec3;
pub mod drawutils;
pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod rtweekend;
pub mod camera;
pub mod material;

pub use vec3::Point;

fn main() {
    println!("Hello, world!");
}
