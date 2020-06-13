mod bmp_converter;
mod camera;
mod hittables;
mod material;
mod ray;
mod rgb;
mod scene;
mod types;
mod vec3;

fn main() {
    println!("Hello, world!");

    let image_width = 384;
    let aspect_ratio = 16. / 9.;
    let mut scene = scene::Scene::new(rgb::RGB {
        r: 0.5,
        g: 0.7,
        b: 1.,
    });
    scene.push(
        Box::new(hittables::Sphere::new(vec3::Point::new(0., 0., -1.), 0.5)),
        Box::new(material::NormalColor::new(rgb::RGB::white())),
    );
    scene.push(
        Box::new(hittables::Sphere::new(
            vec3::Point::new(0., -100.5, -1.),
            100.,
        )),
        Box::new(material::NormalColor::new(rgb::RGB::white())),
    );
    let camera = camera::Camera::new(vec3::Point::origin(), aspect_ratio);
    let picture = camera.cast_rays(image_width, &scene, 100);

    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
