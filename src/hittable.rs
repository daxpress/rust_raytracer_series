use super::ray::Ray;
use super::interval::Interval;
use super::vec3::Vec3;
use super::Point;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitResult>;
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

    #[inline(always)]
    pub fn location(&self) -> &Point {
        &self.location
    }

    #[inline(always)]
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    #[inline(always)]
    pub fn t(&self) -> f64 {
        self.t
    }

    #[inline(always)]
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}
