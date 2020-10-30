use std::cmp::{max, min};

use image;
use rand;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

type Matrix = Vec<Vec<bool>>;

trait MatrixTrait {
    fn is_isolated(&self, p: &Point) -> bool;
}

impl MatrixTrait for Matrix {
    fn is_isolated(&self, p: &Point) -> bool {
        return self[p.x + 1][p.y] && self[p.x - 1][p.y]
            && self[p.x][p.y + 1] && self[p.x][p.y - 1];
    }
}

fn update_cluster_radius(r_cluster: usize, center: usize, p: &Point) -> usize {
    let a_x = max(center, p.x);
    let b_x = min(center, p.x);
    let a_y = max(center, p.y);
    let b_y = min(center, p.y);
    let r_particle = max(a_x - b_x, a_y - b_y);
    return max(r_cluster, r_particle);
}

struct Point {
    x: usize,
    y: usize,
}

struct Box {
    min: usize,
    max: usize,
}

impl Box {
    fn from_center_radius(max_size: usize, center: usize, radius: usize) -> Box {
        let max_size = max_size as i32;
        let center = center as i32;
        let radius = radius as i32;
        let mut min = center - radius;
        let mut max = center + radius;
        if min < 1 {
            min = 1
        } else if min > max_size - 2 {
            min = max_size - 2
        }
        if max < 1 {
            max = 1
        } else if max > max_size - 2 {
            max = max_size - 2
        }
        return Box {min: min as usize, max: max as usize};
    }

    fn sample_point(&self, rg: &mut rand::rngs::SmallRng) -> Point {
        let x = (*rg).gen_range(&self.min, &self.max);
        let y = (*rg).gen_range(&self.min, &self.max);
        return Point { x, y };
    }
    
    fn contains(&self, p: &Point) -> bool {
        return p.x >= self.min && p.x <= self.max && p.y >= self.min && p.y <= self.max
    }
}

fn generate_matrix(size: usize, n_walks: u32) -> Matrix {
    // Define constants and algorithms parameters
    let neighbour_coords = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let vicinity_ratio = 2;
    let escape_ratio = 3;

    // Initialize
    let mut rg_pixels = rand::rngs::SmallRng::from_entropy();
    let mut rg_neighbours = rand::rngs::SmallRng::from_entropy();
    let mut matrix = vec![vec![true; size]; size];
    let center = (size + 1) / 2;
    // Minimal cluster radius considered
    let mut r_cluster = 50;
    matrix[center][center] = false;

    for n in 0..n_walks {
        // Update boxes limits
        let r_vicinity = vicinity_ratio * r_cluster;
        let r_escape = escape_ratio * r_cluster;
        let vicinity_box = Box::from_center_radius(size, center, r_vicinity);
        let escape_box = Box::from_center_radius(size, center, r_escape);

        // Perform random walk
        let mut p = vicinity_box.sample_point(&mut rg_pixels);
        while matrix.is_isolated(&p) {
            // Move the particle
            let displacement = neighbour_coords.choose(&mut rg_neighbours).unwrap();
            p = Point {
                x: (p.x as i32 + displacement.0) as usize,
                y: (p.y as i32 + displacement.1) as usize,
            };

            // Restart when the particle escapes
            if !escape_box.contains(&p) {
                p = vicinity_box.sample_point(&mut rg_pixels);
            }
        }
        matrix[p.x][p.y] = false;
        r_cluster = update_cluster_radius(r_cluster, center, &p);
        println!("Random walk {}/{}.", n + 1, n_walks);
    }
    return matrix;
}

fn convert_matrix(matrix: Matrix) -> image::GrayImage {
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

fn main() {
    let output_filename = "test.png";
    let img_size = 501;
    let n_walks = 20000;

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
