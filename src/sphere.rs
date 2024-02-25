use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;
use super::Point;

struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point, radius: f64) -> Self {
        Sphere {
            center: center.clone(),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b + sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            return false;
        }

        let t = root;
        let location = ray.at(t);
        let normal = (location - self.center) / self.radius;
        *record = HitRecord::new(location, normal, t);
        return true;
    }
}
