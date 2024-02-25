use raytracing_in_one_weekend::raytracer::Raytracer;
fn main() {
    let mut raytracer = Raytracer::new();
    raytracer.init_image();
    raytracer.render_image("test.png");
}
