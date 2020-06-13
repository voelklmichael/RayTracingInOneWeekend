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
    //let image_height = 256;
    let aspect_ratio = 9. / 16.;
    let image_height = (image_width as types::Float * aspect_ratio) as usize;
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
    let picture = camera::cast_rays(image_width, image_height, &scene);

    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
