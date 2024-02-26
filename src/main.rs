use raytracing_in_one_weekend::{
    hittable_list::HittableList, raytracer::Raytracer, sphere::Sphere, Point,
};

use std::rc::Rc;

fn main() {
    let mut raytracer = Raytracer::new();
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    raytracer.render_image(&world);
    raytracer.save_image("test.png");
}
