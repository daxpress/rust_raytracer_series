use std::cmp::min;

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
            return None;
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: scattered,
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let refraction_ratio = if hit_result.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.direction().normalized();
        let cos_theta = f64::min(Vec3::dot(&-unit_dir, hit_result.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            Vec3::reflect(&unit_dir, hit_result.normal())
        }
        else {
            Vec3::refract(&unit_dir, hit_result.normal(), refraction_ratio)
        };

        Some(ScatterResult {
            attenuation: Color::white(),
            scattered_ray: Ray::new(*hit_result.location(), direction)
        })
    }
}
