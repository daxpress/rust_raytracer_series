use super::ray::Ray;

use super::vec3::Vec3;
use super::Point;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitResult>;
}

pub struct HitResult {
    location: Point,
    normal: Vec3,
    t: f64,
}

impl HitResult {
    pub fn new(location: Point, normal: Vec3, t: f64) -> Self {
        HitResult {
            location,
            normal,
            t,
        }
    }
    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}
