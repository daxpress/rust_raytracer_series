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
    pub fn new() -> Self {
        Raytracer {
            data: Vec::new(),
            camera: Camera::new(
                16.0 / 9.0,
                400,
                100,
                50,
                20.0,
                Point::new(-2.0, 2.0, 1.0),
                Point::new(0.0, 0.0, -1.0),
            ),
            //camera: Camera::default(),
            components: 3,
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
