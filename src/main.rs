mod bmp_converter;
mod rbg;
mod types;

fn main() {
    println!("Hello, world!");

    let image_width = 256;
    let image_height = 256;

    let mut picture = Vec::with_capacity(image_height);

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
        }
        picture.push(row);
    }
    bmp_converter::write_file("sample.bmp", &picture).expect("Failed to write image");
}
