use std::f64::INFINITY;

use super::color::Color;
use super::hittable::Hittable;
use super::interval::Interval;
use super::vec3::Vec3;
use super::Point;

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn empty() -> Self {
        Ray {
            origin: Point::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    #[inline(always)]
    pub fn origin(&self) -> Point {
        self.origin
    }

    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &dyn Hittable, depth: u32) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        // interval starts from 0.001 to hack away the shadow acne problem
        let hit = world.hit(&self, Interval::new(0.001, INFINITY));
        match hit {
            None => (),
            Some(result) => {
                let direction = Vec3::rand_on_hemisphere(result.normal());
                return Ray::new(*result.location(), direction).color(world, depth-1) * 0.5;
            }
        }
        // background
        // first map a from -1..1 to 0..1
        let a = 0.5 * (self.direction.normalized().y() + 1.0);
        // now lerp the colors
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }
}
