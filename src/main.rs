use raytracing_in_one_weekend::raytracer::Raytracer;
fn main() {
    let mut raytracer = Raytracer::new();
    println!("{:#?}", raytracer);
    raytracer.init_image();
    raytracer.render_image("test.png");
}
