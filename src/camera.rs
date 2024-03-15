use crate::interval::Interval;
use crate::utilities::deg_to_rad;

use super::color::Color;
use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::Vec3;
use super::Point;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    center: Point,
    viewport: Viewport,
    samples: u32,
    max_depth: u32,
    v_fov: f64,
    lookfrom: Point,
    lookat: Point,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples: u32,
        max_depth: u32,
        v_fov: f64,
        lookfrom: Point,
        lookat: Point,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let vup = Vec3::up();
        let center = lookfrom;
        let theta = deg_to_rad(v_fov);
        let h = f64::tan(theta * 0.5);

        let w = (lookfrom - lookat).normalized();
        let u = Vec3::cross(&vup, &w).normalized();
        let v = Vec3::cross(&w, &u);

        let viewport = Viewport::new(aspect_ratio, h, image_width, focus_dist, center, u, v, w);

        let defocus_radius = focus_dist * f64::tan(deg_to_rad(defocus_angle * 0.5));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            center,
            viewport,
            samples,
            max_depth,
            v_fov,
            lookat,
            lookfrom,
            u,
            v,
            w,
            vup,
            focus_dist,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn default() -> Camera {
        Camera::new(
            16.0 / 9.0,
            400,
            10,
            10,
            90.0,
            Point::zero(),
            Point::new(0.0, 0.0, -1.0),
            10.0,
            0.0,
        )
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
            println!("\rScanlines remaining: {} ", self.height() - j);

            for i in 0..self.width() {
                let pixel_color = (0..self.samples)
                    .map(|_| {
                        let ray = self.get_ray(i, j);
                        ray.color(world, self.max_depth)
                    })
                    .reduce(|acc, color| (acc + color))
                    .unwrap_or(Vec3::black());
                self.write_pixel(&pixel_color, self.samples, image_data, (i, j));
            }
        }

        println!("\rDone.");
    }

    pub fn render_parallel(&self, world: &(dyn Hittable + Sync + Send), image_data: &mut Vec<u8>) {
        for j in 0..self.height() {
            println!("\rScanlines remaining: {} ", self.height() - j);
            for i in 0..self.width() {
                let pixel_color = (0..self.samples)
                    .into_par_iter()
                    .map(|_| {
                        let ray = self.get_ray(i, j);
                        ray.color(world, self.max_depth)
                    })
                    .reduce(|| (Vec3::zero()), |a, b| (a + b));
                self.write_pixel(&pixel_color, self.samples, image_data, (i, j));
            }
        }

        println!("\rDone.");
    }

    fn write_pixel(
        &self,
        color: &Color,
        samples: u32,
        image_data: &mut Vec<u8>,
        uv: (usize, usize),
    ) {
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

        let index = uv.1 * (self.width() * 3) + uv.0 * 3;

        image_data[index + 0] = r as u8;
        image_data[index + 1] = g as u8;
        image_data[index + 2] = b as u8;
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.viewport.pixel_00
            + (i as f64 * self.viewport.pixel_delta_u)
            + (j as f64 * self.viewport.pixel_delta_v);
        let sample = pixel_center - self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        use super::utilities;
        let px = -0.5 + utilities::rand();
        let py = -0.5 + utilities::rand();

        (px * self.viewport.pixel_delta_u) + (py * self.viewport.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Point::rand_in_unit_disk();
        self.center + (*p.x() * self.defocus_disk_u) + (*p.y() * self.defocus_disk_v)
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
        h: f64,
        image_width: usize,
        focus_dist: f64,
        center: Point,
        u: Vec3,
        v: Vec3,
        w: Vec3,
    ) -> Viewport {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        if image_height < 1 {
            image_height = 1;
        }
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // get the viewport plane position, then move it toi the center
        let viewport_upper_left = center - focus_dist * w - viewport_u * 0.5 - viewport_v * 0.5;

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
