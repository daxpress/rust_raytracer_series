use super::hittable::{HitResult, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;
use super::Point;
use super::interval::Interval;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitResult> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return None
            }
        }

        let t = root;
        let location = ray.at(t);
        let normal = (location - self.center) / self.radius;
        return Some(HitResult::new(ray, location, normal, t));
    }
}
