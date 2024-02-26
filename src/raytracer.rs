use stb::image_write::stbi_write_png;
use std::{ffi::CString, fmt::Display};
use super::hittable::Hittable;
use super::camera::Camera;

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
            self.camera.width(), self.camera.height(), self.components
        )
    }
}
