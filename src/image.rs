use image;
use crate::matrix;

pub fn convert_matrix(matrix: matrix::RankMatrix, mode: u32, max_value:u32) -> image::DynamicImage {
    return if mode == 0 {
        image::DynamicImage::ImageLuma8(convert_matrix_bw(matrix))
    } else if mode == 1 {
        image::DynamicImage::ImageRgb8(convert_matrix_color(matrix, max_value))
    } else {
        panic!()
    };
}

fn convert_matrix_color(matrix: matrix::RankMatrix, max_val: u32) -> image::RgbImage {
    let width = matrix.len();
    let height = matrix[0].len();
    let mut img = image::RgbImage::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            if matrix[x][y] != 0 {
                let p = image::Rgb([255, (255 - matrix[x][y] * 255 / max_val) as u8, 0]);
                img.put_pixel(x as u32, y as u32, p);
            }
        }
    }
    return img;
}

fn convert_matrix_bw(matrix: matrix::RankMatrix) -> image::GrayImage {
    let width = matrix.len();
    let height = matrix[0].len();
    let mut img = image::GrayImage::new(width as u32, height as u32);
    let hot_pixel = image::Luma([0]);
    let cold_pixel = image::Luma([255]);
    for x in 0..width {
        for y in 0..height {
            if matrix[x][y] != 0 {
                img.put_pixel(x as u32, y as u32, hot_pixel);
            } else {
                img.put_pixel(x as u32, y as u32, cold_pixel);
            }
        }
    }
    return img;
}
