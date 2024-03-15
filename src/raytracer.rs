use crate::Point;

use super::camera::Camera;
use super::hittable::Hittable;
use stb::image_write::stbi_write_png;
use std::{ffi::CString, fmt::Display};

#[derive(Debug)]
pub struct Raytracer {
    data: Vec<u8>,
    camera: Camera,
    components: i32,
}

impl Raytracer {
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
        let camera = Camera::new(
            aspect_ratio,
            image_width,
            samples,
            max_depth,
            v_fov,
            lookfrom,
            lookat,
            focus_dist,
            defocus_angle,
        );
        let components = 3;
        let data = vec![0; camera.width() * camera.height() * components];

        Raytracer {
            data,
            camera,
            //camera: Camera::default(),
            components: components as i32,
        }
    }

    pub fn render_image(&mut self, world: &dyn Hittable) {
        self.camera.render(world, &mut self.data)
    }

    pub fn save_image(&self, filename: &str) {
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
}

impl Display for Raytracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "width: {}, height: {}, components: {}",
            self.camera.width(),
            self.camera.height(),
            self.components
        )
    }
}
