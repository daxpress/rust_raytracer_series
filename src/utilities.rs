const PI: f64 = 3.1415926535897932385;

#[inline(always)]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}