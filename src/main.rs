mod bmp_converter;
mod camera;
mod hittables;
mod material;
mod onb;
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

    use hittables::Sphere;
    scene.push(
        Sphere::new(vec3::Point::new(0., -1000., -0.), 1000.),
        material::Lambertian::new(rgb::RGB::new(0.5, 0.5, 0.5)),
    );

    for a in -11..11 {
        for b in -11..11 {
            use crate::types::{generate_random, Float};
            let center = vec3::Point::new(
                a as Float + 0.9 * generate_random(),
                0.2,
                b as Float + 0.9 * generate_random(),
            );

            if (center.clone() - vec3::Point::new(4., 0.2, 0.)).length() > 0.9 {
                let choose_mat = generate_random();

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rgb::RGB::random(0., 1.).clone() * rgb::RGB::random(0., 1.);
                    scene.push(Sphere::new(center, 0.2), material::Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rgb::RGB::random(0.5, 1.);
                    let fuzz = crate::types::generate_random_in_between(0.0, 0.5);
                    scene.push(
                        Sphere::new(center, 0.2),
                        material::FuzzyMetal::new(albedo, fuzz),
                    );
                } else {
                    // glass
                    scene.push(Sphere::new(center, 0.2), material::Dielectric::new(1.5));
                }
            }
        }
    }

    scene.push(
        Sphere::new(vec3::Point::new(0., 1., 0.), 1.),
        material::Dielectric::new(1.5),
    );

    scene.push(
        Sphere::new(vec3::Point::new(-4., 1., 0.), 1.),
        material::Lambertian::new(rgb::RGB::new(0.4, 0.2, 0.1)),
    );

    scene.push(
        Sphere::new(vec3::Point::new(4., 1., 0.), 1.),
        material::FuzzyMetal::new(rgb::RGB::new(0.7, 0.6, 0.5), 0.0),
    );

    let center = vec3::Point::new(13., 2., 3.);
    let looking_towards = vec3::Point::new(0., 0., 0.);
    let view_up = vec3::Direction::new(0., 1., 0.);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let vertical_field_of_view = 20.;
    let camera = camera::Camera::new(
        center,
        looking_towards,
        vertical_field_of_view,
        view_up,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    let picture = camera.cast_rays(image_width, &scene, rays_per_pixel, recursion_depth);

    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
