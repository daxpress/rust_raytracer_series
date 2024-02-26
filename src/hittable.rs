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
    front_face: bool,
}

impl HitResult {
    pub fn new(ray: &Ray, location: Point, normal: Vec3, t: f64) -> Self {
        let front_face = Vec3::dot(&ray.direction(), &normal) < 0.0;
        let mut normal = normal;
        if !front_face {
            normal = -normal;
        }
        HitResult {
            location,
            normal,
            t,
            front_face,
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

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}
