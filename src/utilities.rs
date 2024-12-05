const PI: f64 = 3.1415926535897932385;

#[inline(always)]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

use std::time::Duration;

use rand::*;

pub fn rand() -> f64 {
    random()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn print_duration(duration: Duration) {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 60 / 60;
    println!("{}:{:02}:{:02}", hours, minutes, seconds)
}