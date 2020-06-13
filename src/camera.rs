use crate::ray::{Direction, Float, Point, Ray};
use crate::rbg::RGB;

fn ray_color(ray: Ray) -> RGB {
    let dir = ray.direction().unit_vector();
    let t = 0.5 * (dir.y() + 1.);
    let s = 1. - t;
    RGB {
        r: s + 0.5 * t,
        g: s + 0.7 * t,
        b: s + 1.0 * t,
    }
}

pub fn cast_rays(image_width: usize, image_height: usize) -> Vec<Vec<RGB>> {
    let aspect_ratio = image_width as Float / image_height as Float;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::origin();
    let horizontal = Direction::new(viewport_width, 0., 0.);
    let vertical = Direction::new(0., viewport_height, 0.);
    let lower_left_corner = origin.clone()
        + horizontal.clone() / -2.0
        + vertical.clone() / -2.0
        + Direction::new(0., 0., -focal_length);

    let mut picture = Vec::with_capacity(image_height);

    let progress_every = 1000;
    let mut pb = pbr::ProgressBar::new(image_height as u64 * image_width as u64 / progress_every);
    pb.format("╢▌▌░╟");
    let mut progress = 0;
    for row_index in 0..image_height {
        let mut row = Vec::with_capacity(image_width);
        for colum_index in 0..image_width {
            let u = (colum_index as Float) / ((image_width - 1) as Float);
            let v = (row_index as Float) / ((image_height - 1) as Float);
            let direction =
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                    - origin.clone();
            let ray = Ray::new(origin.clone(), direction);
            let rbg = ray_color(ray);
            row.push(rbg);
            progress += 1;
            if progress == progress_every {
                pb.inc();
                progress = 0;
            }
        }
        picture.push(row);
    }
    pb.finish_print("done");
    picture
}

/*

#include <iostream>



int main() {
    const auto aspect_ratio = 16.0 / 9.0;
    const int image_width = 384;
    const int image_height = static_cast<int>(image_width / aspect_ratio);

    std::cout << "P3\n" << image_width << " " << image_height << "\n255\n";

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    auto origin = point3(0, 0, 0);
    auto horizontal = vec3(viewport_width, 0, 0);
    auto vertical = vec3(0, viewport_height, 0);
    auto lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);

    for (int j = image_height-1; j >= 0; --j) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (int i = 0; i < image_width; ++i) {
            auto u = double(i) / (image_width-1);
            auto v = double(j) / (image_height-1);
            ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            color pixel_color = ray_color(r);
            write_color(std::cout, pixel_color);
        }
    }

    std::cerr << "\nDone.\n";
}*/
