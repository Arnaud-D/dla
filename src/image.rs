use image;
use crate::matrix;

pub fn convert_matrix(matrix: matrix::Matrix) -> image::GrayImage {
    let width = matrix.len();
    let height = matrix[0].len();
    let mut img = image::GrayImage::new(width as u32, height as u32);
    let hot_pixel = image::Luma([0]);
    let cold_pixel = image::Luma([255]);
    for x in 0..width {
        for y in 0..height {
            if !matrix[x][y] {
                img.put_pixel(x as u32, y as u32, hot_pixel);
            } else {
                img.put_pixel(x as u32, y as u32, cold_pixel);
            }
        }
    }
    return img;
}
