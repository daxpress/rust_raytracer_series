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
        let hit = self.sphere_hit(&Point::new(0.0, 0.0, -1.0), 0.5);
        if hit > 0.0 {
            let normal = (self.at(hit) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }
        // first map a from -1..1 to 0..1
        let a = 0.5 * (self.direction.normalized().y() + 1.0);
        // now lerp the colors
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn sphere_hit(&self, center: &Point, radius: f64) -> f64 {
        let oc = self.origin - *center;
        // dot of a vector by itself is the squared length of that vector
        let a = self.direction.len_squared();
        let half_b = Vec3::dot(&oc, &self.direction);
        let c = oc.len_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        else {
            return (-half_b - discriminant.sqrt()) / a;
        }
    }
}