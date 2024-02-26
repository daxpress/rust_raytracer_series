use crate::interval::Interval;

use super::color::Color;
use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::Vec3;
use super::Point;
#[derive(Debug)]
pub struct Camera {
    center: Point,
    focal_length: f64,
    viewport: Viewport,
    samples: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples: u32, max_depth: u32) -> Self {
        let focal_length = 1.0;
        let center = Point::zero();
        let viewport = Viewport::new(aspect_ratio, 2.0, image_width, 1.0, center);

        Camera {
            center,
            focal_length,
            viewport,
            samples,
            max_depth,
        }
    }

    pub fn default() -> Camera {
        Camera::new(16.0 / 9.0, 400, 10, 10)
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

    pub fn render(&self, world: &dyn Hittable, image_data: &mut Vec<u8>) {
        for j in 0..self.height() {
            //println!("\rScanlines remaining: {} ", self.camera.height() - j);

            for i in 0..self.width() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray.color(world, self.max_depth);
                }
                self.write_pixel(&pixel_color, self.samples, image_data);
            }
        }

        println!("\rDone.");
    }

    fn write_pixel(&self, color: &Color, samples: u32, image_data: &mut Vec<u8>) {
        let scale = 1.0 / samples as f64;

        let intesity: Interval = Interval::new(0.0, 1.0);

        let color = Color::linear_to_gamma(&Color::new(
            *color.r() * scale,
            *color.g() * scale,
            *color.b() * scale,
        ));

        let r = 255f64 * intesity.clamp(*color.r());
        let g = 255f64 * intesity.clamp(*color.g());
        let b = 255f64 * intesity.clamp(*color.b());

        image_data.push(r as u8);
        image_data.push(g as u8);
        image_data.push(b as u8);
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.viewport.pixel_00
            + (i as f64 * self.viewport.pixel_delta_u)
            + (j as f64 * self.viewport.pixel_delta_v);
        let sample = pixel_center - self.pixel_sample_square();

        let ray_direction = sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        use super::utilities;
        let px = -0.5 + utilities::rand();
        let py = -0.5 + utilities::rand();

        (px * self.viewport.pixel_delta_u) + (py * self.viewport.pixel_delta_v)
    }
}

#[derive(Debug)]
struct Viewport {
    pub image_width: usize,
    pub image_height: usize,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_00: Vec3,
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
        }
    }
}
