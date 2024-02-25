use stb::image_write::stbi_write_png;
use std::{ffi::CString, fmt::Display};

use super::camera::Camera;
use super::color::Color;
use super::ray::Ray;

#[derive(Debug)]
pub struct Raytracer {
    data: Vec<u8>,
    camera: Camera,
    components: i32
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer {
            data: Vec::new(),
            camera: Camera::default(),
            components: 3
        }
    }

    pub fn init_image(&mut self) {
        for j in 0..self.camera.height() {
            //println!("\rScanlines remaining: {} ", self.camera.height() - j);

            for i in 0..self.camera.width() {
                let pixel_center = self.camera.viewport().get_pixel_center(i as i32, j as i32);
                let ray_direction = pixel_center - self.camera.center();

                let ray = Ray::new(self.camera.center(), ray_direction);

                let color = ray.get_color();
                self.write_pixel(&color)
            }
        }

        println!("\rDone.");
    }

    pub fn render_image(&self, filename: &str) {
        let cstr = CString::new(filename).unwrap();
        stbi_write_png(
            &cstr,
            self.camera.width() as i32,
            self.camera.height() as i32,
            self.components,
            &self.data,
            3 * self.camera.width() as i32,
        );
    }

    fn write_pixel(&mut self, color: &Color) {
        self.data.push((255f64 * *color.r()) as u8);
        self.data.push((255f64 * *color.g()) as u8);
        self.data.push((255f64 * *color.b()) as u8);
    }
}

impl Display for Raytracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "width: {}, height: {}, components: {}",
            self.camera.width(), self.camera.height(), self.components
        )
    }
}
