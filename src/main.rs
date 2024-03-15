use raytracing_series::{
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    raytracer::Raytracer,
    sphere::Sphere,
    utilities::{rand, rand_range},
    Point,
};

use std::sync::Arc;

fn main() {
    let mut raytracer = Raytracer::new(
        16.0 / 9.0,
        1200,
        100,
        50,
        20.0,
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        10.0,
        0.6,
    );

    let mut world = HittableList::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&ground_mat) as Arc<dyn Material>,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Point::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_mat: Arc<dyn Material>;

                match choose_mat {
                    _ if choose_mat < 0.8 => {
                        let albedo = Color::rand() * Color::rand();
                        sphere_mat = Arc::new(Lambertian::new(albedo));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    }
                    _ if choose_mat < 0.95 => {
                        let albedo = Color::rand_range(0.5, 1.0);
                        let fuzz = rand_range(0.0, 0.5);
                        sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    }
                    _ => {
                        sphere_mat = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    }
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3)));

    // raytracer.render_image(&world);
    raytracer.render_image_parallel(&world);
    raytracer.save_image("test.png");
}
