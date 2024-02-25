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
        if self.sphere_hit(&Point::new(0.0, 0.0, -1.0), 0.5) {
            return Color::red();
        }
        // first map a from -1..1 to 0..1
        let a = 0.5 * (self.direction.normalized().y() + 1.0);
        // now lerp the colors
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn sphere_hit(&self, center: &Point, radius: f64) -> bool {
        let oc = self.origin - *center;
        let a = Vec3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vec3::dot(&oc, &self.direction);
        let c = Vec3::dot(&oc, &oc) - radius * radius;
        let discriminant = b*b - 4.0 * a * c;
        discriminant >= 0.0
    }
}