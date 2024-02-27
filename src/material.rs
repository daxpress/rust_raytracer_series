use crate::color::Color;
use crate::vec3::Vec3;
use crate::{hittable::HitResult, ray::Ray};

pub struct ScatterResult {
    attenuation: Color,
    scattered_ray: Ray,
}

impl ScatterResult {
    pub fn new(attenuation: Color, scattered_ray: Ray) -> Self {
        ScatterResult {
            attenuation,
            scattered_ray,
        }
    }

    #[inline(always)]
    pub fn attenuation(&self) -> Color {
        self.attenuation
    }

    #[inline(always)]
    pub fn scattered_ray(&self) -> &Ray {
        &self.scattered_ray
    }
}
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let mut scatter_dir = *hit_result.normal() + Vec3::rand_unit();
        if scatter_dir.near_zero() {
            scatter_dir = *hit_result.normal();
        }

        Some(ScatterResult::new(
            self.albedo,
            Ray::new(*hit_result.location(), scatter_dir),
        ))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflected = Vec3::reflect(&ray.direction().normalized(), &hit_result.normal());
        let scattered = Ray::new(
            *hit_result.location(),
            reflected + self.fuzz * Vec3::rand_unit(),
        );
        if Vec3::dot(&scattered.direction(), hit_result.normal()) < 0.0 {
            return None
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: scattered,
        })
    }
}
