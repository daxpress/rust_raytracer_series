const PI: f64 = 3.1415926535897932385;

#[inline(always)]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

use rand::*;

pub fn rand() -> f64 {
    thread_rng().gen()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}