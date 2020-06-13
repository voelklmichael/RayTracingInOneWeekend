use crate::rbg::{RGBu8, RGB};
use bmp::Image;

pub fn convert_to_bmp(picture: &[Vec<RGB>]) -> Image {
    let rows = picture.len();
    if let Some(first) = picture.first() {
        let columns = first.len();
        let mut image = Image::new(columns as u32, rows as u32);
        for (row_index, column) in picture.into_iter().rev().enumerate() {
            for (column_index, rbg) in column.into_iter().enumerate() {
                let RGBu8 { r, g, b } = rbg.round();
                image.set_pixel(
                    column_index as u32,
                    row_index as u32,
                    bmp::Pixel::new(r, g, b),
                );
            }
        }
        image
    } else {
        Image::new(0, 0)
    }
}

pub fn write_file(filename: &str, picture: &[Vec<RGB>]) -> Result<(), std::io::Error> {
    let image = convert_to_bmp(picture);
    image.save(filename)
}
