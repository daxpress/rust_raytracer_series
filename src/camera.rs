use super::vec3::Vec3;
use super::Point;


#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f64,
    center: Point,
    focal_length: f64,
    viewport: Viewport,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let focal_length = 1.0;
        let center = Point::zero();

        let viewport = Viewport::new(aspect_ratio, 2.0, image_width, 1.0, center);

        Camera {
            aspect_ratio,
            center,
            focal_length,
            viewport,
        }
    }

    pub fn default() -> Camera {
        Camera::new(16.0 / 9.0, 400)
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn height(&self) -> usize {
        self.viewport.image_height
    }

    pub fn width(&self) -> usize {
        self.viewport.image_width
    }
}

#[derive(Debug)]
pub struct Viewport {
    image_width: usize,
    image_height: usize,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00: Vec3,
}

impl Viewport {
    pub fn new(aspect_ratio: f64, viewport_height: f64, image_width: usize, focal_length: f64, center: Point) -> Viewport {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        if image_height < 1 {
            image_height = 1;
        }
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // get the viewport plane position, then move it toi the center
        let viewport_upper_left =
            center - Point::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;

        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Viewport {
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00
        }
    }

    pub fn get_pixel_center(&self, x: i32, y: i32) -> Vec3 {
        self.pixel_00 + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v)
    }
}
