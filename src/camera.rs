use crate::rgb::RGB;
use crate::scene::{Direction, Float, Point, Ray, Scene};

pub struct Camera {
    center: Point,
    lower_left_corner: Point,
    horizontal: Direction,
    vertical: Direction,
    aspect_ratio: Float,
}
impl Camera {
    pub fn new(center: Point, aspect_ratio: Float) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = Direction::new(viewport_width, 0., 0.);
        let vertical = Direction::new(0., viewport_height, 0.);
        let lower_left_corner = center.clone()
            + horizontal.clone() / -2.0
            + vertical.clone() / -2.0
            + Direction::new(0., 0., -focal_length);
        Self {
            center,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
        }
    }
    pub fn cast_ray(&self, u: Float, v: Float) -> Ray {
        let direction = self.lower_left_corner.clone()
            + self.horizontal.clone() * u
            + self.vertical.clone() * v
            - self.center.clone();
        Ray::new(self.center.clone(), direction)
    }

    pub fn cast_rays(
        &self,
        image_width: usize,
        scene: &Scene,
        rays_per_pixel: usize,
    ) -> Vec<Vec<RGB>> {
        let image_height = (image_width as Float / self.aspect_ratio) as usize;

        let mut picture = Vec::with_capacity(image_height);

        let progress_every = 1000;
        let mut pb =
            pbr::ProgressBar::new(image_height as u64 * image_width as u64 / progress_every);
        pb.format("╢▌▌░╟");
        let mut progress = 0;
        for row_index in 0..image_height {
            let mut row = Vec::with_capacity(image_width);
            for colum_index in 0..image_width {
                let mut rgb = RGB::black();
                for _ in 0..rays_per_pixel {
                    use crate::types::generate_random;
                    let u =
                        (colum_index as Float + generate_random()) / ((image_width - 1) as Float);
                    let v =
                        (row_index as Float + generate_random()) / ((image_height - 1) as Float);
                    let ray = self.cast_ray(u, v);
                    rgb += if let Some((hit, material)) = scene.hit(&ray, 0., Float::INFINITY) {
                        material.get_color(&hit)
                    } else {
                        scene.background(&ray)
                    };
                }
                rgb.normalize(rays_per_pixel);

                row.push(rgb);
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
