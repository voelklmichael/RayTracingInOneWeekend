mod bmp_converter;
mod rbg;
mod types;

fn main() {
    println!("Hello, world!");

    let image_width = 256;
    let image_height = 256;

    let mut picture = Vec::with_capacity(image_height);

    let progress_every = 1000;
    let mut pb = pbr::ProgressBar::new(image_height as u64 * image_width as u64 / progress_every);
    pb.format("╢▌▌░╟");
    let mut progress = 0;
    for row_index in 0..image_height {
        let mut row = Vec::with_capacity(image_width);
        for colum_index in 0..image_width {
            use rbg::Float;
            let rbg = rbg::RGB {
                r: colum_index as Float / ((image_width - 1) as Float),
                g: row_index as Float / ((image_height - 1) as Float),
                b: 0.25,
            };
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
    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
    println!("File written");
}
