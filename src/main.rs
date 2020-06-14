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

    let recursion_depth = 50;
    let rays_per_pixel = 100;
    let image_width = 384;
    let aspect_ratio = 16. / 9.;
    let mut scene = scene::Scene::new(rgb::RGB {
        r: 0.5,
        g: 0.7,
        b: 1.,
    });
    scene.push(
        Box::new(hittables::Sphere::new(vec3::Point::new(1., 0., -1.), 0.5)),
        Box::new(material::FuzzyMetal::new(rgb::RGB::new(0.8, 0.6, 0.2), 0.3)),
    );
    scene.push(
        Box::new(hittables::Sphere::new(vec3::Point::new(-1., 0., -1.), 0.5)),
        Box::new(material::FuzzyMetal::new(rgb::RGB::new(0.8, 0.8, 0.8), 1.0)),
    );

    scene.push(
        Box::new(hittables::Sphere::new(vec3::Point::new(0., 0., -1.), 0.5)),
        Box::new(material::Lambertian::new(rgb::RGB::new(0.7, 0.3, 0.3))),
    );
    scene.push(
        Box::new(hittables::Sphere::new(
            vec3::Point::new(0., -100.5, -1.),
            100.,
        )),
        Box::new(material::Lambertian::new(rgb::RGB::new(0.8, 0.8, 0.0))),
    );

    let camera = camera::Camera::new(vec3::Point::origin(), aspect_ratio);
    let picture = camera.cast_rays(image_width, &scene, rays_per_pixel, recursion_depth);

    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
