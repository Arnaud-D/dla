use image;
use rand;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

fn has_no_neighbours(matrix : &Vec<Vec<bool>>, x : usize, y: usize) -> bool {
    matrix[x + 1][y] && matrix[x - 1][y] && matrix[x][y + 1] && matrix[x][y - 1]
}

fn generate_matrix(size: usize, n_walks: u32) -> Vec<Vec<bool>> {
    let mut matrix = vec![vec![true; size]; size];
    let midpoint = (size + 1) / 2;
    matrix[midpoint][midpoint] = false;

    let mut rg_pixels = rand::rngs::SmallRng::from_entropy();
    let mut rg_neighbours = rand::rngs::SmallRng::from_entropy();
    let mut x;
    let mut y;
    let neighbour_coords= [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let idx_min = 1;
    let idx_max = size - 2;
    for n in 0..n_walks {
        x = rg_pixels.gen_range(idx_min, idx_max);
        y = rg_pixels.gen_range(idx_min, idx_max);
        while has_no_neighbours(&matrix, x, y) {
            let coord = neighbour_coords.choose(&mut rg_neighbours).unwrap();

            x = (x as i32 + coord.0) as usize;
            if x > idx_max {
                x = idx_max;
            } else if x < idx_min {
                x = idx_min;
            }

            y = (y as i32 + coord.1) as usize;
            if y > idx_max {
                y = idx_max;
            } else if y < idx_min {
                y = idx_min;
            }
        }
        matrix[x][y] = false;
        println!("Random walk {}/{}.", n + 1, n_walks);
    }
    return matrix;
}

fn convert_matrix(matrix: Vec<Vec<bool>>) -> image::GrayImage {
    let width = matrix.len();
    let height = matrix[0].len();
    let mut img = image::GrayImage::new(width as u32, height as u32);
    let hot_pixel = image::Luma([255]);
    for x in 0..width {
        for y in 0..height {
            if !matrix[x][y] {
                img.put_pixel(x as u32, y as u32, hot_pixel);
            }
        }
    }
    return img;
}

fn main() {
    let output_filename = "test.png";
    let img_size = 501;
    let n_walks = 10000;

    let start_time = std::time::SystemTime::now();

    let matrix = generate_matrix(img_size, n_walks);
    let img = convert_matrix(matrix);

    let elapsed = match start_time.elapsed() {
        Ok(s) => s,
        Err(_) => panic!("wow"),
    };
    println!("Executed in {} seconds.", elapsed.as_secs());

    match img.save(output_filename) {
        Ok(_) => println!("File `{}` saved successfully.", output_filename),
        Err(_) => println!("An error occured when saving the image. The file `{}` might be write-protected.", output_filename),
    }
}
