use super::vec3::Vec3;
use super::Point;
use super::hittable::Hittable;
use super::ray::Ray;
use super::color::Color;
#[derive(Debug)]
pub struct Camera {
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
            center,
            focal_length,
            viewport,
        }
    }

    pub fn default() -> Camera {
        Camera::new(16.0 / 9.0, 400)
    }

    #[inline(always)]
    pub fn center(&self) -> Point {
        self.center
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.viewport.image_height
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.viewport.image_width
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.viewport.aspect_ratio = aspect_ratio
    }

    pub fn set_image_width(&mut self, width: usize) {
        self.viewport.image_width = width
    }

    pub fn render(&self, world: &dyn Hittable, image_data: &mut Vec<u8>) {
        for j in 0..self.height() {
            //println!("\rScanlines remaining: {} ", self.camera.height() - j);

            for i in 0..self.width() {
                let pixel_center = self.viewport.get_pixel_center(i as i32, j as i32);
                let ray_direction = pixel_center - self.center();

                let ray = Ray::new(self.center(), ray_direction);

                let color = ray.color(world);
                self.write_pixel(&color, image_data);
            }
        }

        println!("\rDone.");
    }

    fn write_pixel(&self, color: &Color, image_data: &mut Vec<u8>) {
        image_data.push((255f64 * *color.r()) as u8);
        image_data.push((255f64 * *color.g()) as u8);
        image_data.push((255f64 * *color.b()) as u8);
    }

    
}

#[derive(Debug)]
struct Viewport {
    pub image_width: usize,
    pub image_height: usize,
    pub viewport_width: f64,
    pub viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00: Vec3,
    pub aspect_ratio: f64,
}

impl Viewport {
    pub fn new(
        aspect_ratio: f64,
        viewport_height: f64,
        image_width: usize,
        focal_length: f64,
        center: Point,
    ) -> Viewport {
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
            pixel_00,
            aspect_ratio
        }
    }

    pub fn get_pixel_center(&self, x: i32, y: i32) -> Vec3 {
        self.pixel_00 + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v)
    }   
}
