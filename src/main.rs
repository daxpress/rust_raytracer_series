use raytracing_in_one_weekend::{
    color::Color, hittable_list::HittableList, material::{Dielectric, Lambertian, Metal}, raytracer::Raytracer, sphere::Sphere, Point,
};

use std::rc::Rc;

fn main() {
    let mut raytracer = Raytracer::new();
    let mut world = HittableList::new();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_mat = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    let right_mat = Rc::new(Dielectric::new(1.5));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        center_mat.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        right_mat.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        right_mat.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        left_mat.clone(),
    )));

    raytracer.render_image(&world);
    raytracer.save_image("test.png");
}
