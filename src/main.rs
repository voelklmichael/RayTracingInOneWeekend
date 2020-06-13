mod bmp_converter;
mod camera;
mod ray;
mod rbg;
mod types;
mod vec3;

fn main() {
    println!("Hello, world!");

    let image_width = 384;
    //let image_height = 256;
    let aspect_ratio = 16. / 9.;
    let image_height = (image_width as types::Float * aspect_ratio) as usize;

    let picture = camera::cast_rays(image_width, image_height);

    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
