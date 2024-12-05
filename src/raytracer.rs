use crate::{hittable_list::HittableList, Point};

use super::camera::Camera;
use crate::utilities::print_duration;

use std::{fmt::Display, time::SystemTime};

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

    pub fn render(&mut self, world: &HittableList, options: RaytracerOptions) {
        match options.execution_method() {
            ExecutionMethod::Single => self.render_image(world),
            ExecutionMethod::Parallel => self.render_image_parallel(world),
        }
    }

    fn render_image(&mut self, world: &HittableList) {
        let start = SystemTime::now();
        self.camera.render(world, &mut self.data);
        let end = start.elapsed().unwrap();
        print!("Rendering duration: ");
        print_duration(end);
    }

    fn render_image_parallel(&mut self, world: &HittableList) {
        let start = SystemTime::now();
        self.camera.render_parallel(world, &mut self.data);
        let end = start.elapsed().unwrap();
        print!("Rendering duration: ");
        print_duration(end);
    }

    pub fn save_image(&self, filename: &str) {
        image::save_buffer(
            filename,
            &self.data,
            self.camera.width() as u32,
            self.camera.height() as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
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

#[derive(Clone, Copy)]
pub enum ExecutionMethod {
    Single,
    Parallel,
}

pub struct RaytracerOptions {
    execution_method: ExecutionMethod,
}

impl RaytracerOptions {
    pub fn new(execution_method: ExecutionMethod) -> Self {
        RaytracerOptions { execution_method }
    }

    pub fn execution_method(&self) -> ExecutionMethod {
        self.execution_method
    }
}
