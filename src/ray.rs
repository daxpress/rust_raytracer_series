use super::Point;
use super::vec3::Vec3;
use super::color::Color;

pub struct Ray {
    origin: Point,
    direction: Vec3
}

impl Ray {
    pub fn empty() -> Self {
        Ray {
            origin: Point::zero(),
            direction: Vec3::zero()
        }
    }

    pub fn new(origin: &Point, direction: &Vec3) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone()
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn get_color(&self) -> Color {
        // first map a from -1..1 to 0..1
        let a = 0.5 * (self.direction.normalized().y() + 1.0);
        // now lerp the colors
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }
}