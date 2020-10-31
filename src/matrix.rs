use std::cmp::{max, min};

use rand;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

pub type BoolMatrix = Vec<Vec<bool>>;
pub type RankMatrix = Vec<Vec<u32>>;

trait BoolMatrixTrait {
    fn is_isolated(&self, p: &Point) -> bool;
    fn sample_point(&self, rg: &mut rand::rngs::SmallRng, square_box:&Box) -> Point;
    fn sample_free_point(&self, rg: &mut rand::rngs::SmallRng, square_box:&Box) -> Point;
}

impl BoolMatrixTrait for BoolMatrix {
    fn is_isolated(&self, p: &Point) -> bool {
        return self[p.x + 1][p.y] && self[p.x - 1][p.y]
            && self[p.x][p.y + 1] && self[p.x][p.y - 1];
    }

    fn sample_point(&self, rg: &mut rand::rngs::SmallRng, square_box:&Box) -> Point {
        Point {
            x: (*rg).gen_range(square_box.min, square_box.max),
            y: (*rg).gen_range(square_box.min, square_box.max),
        }
    }

    fn sample_free_point(&self, rg: &mut rand::rngs::SmallRng, square_box:&Box) -> Point {
        let mut p = self.sample_point(rg, square_box);
        while !self.is_isolated(&p) || !self[p.x][p.y] {
            p = self.sample_point(rg, square_box);
        }
        return p;
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
        return Box { min: min as usize, max: max as usize };
    }

    fn contains(&self, p: &Point) -> bool {
        return p.x >= self.min && p.x <= self.max && p.y >= self.min && p.y <= self.max;
    }
}

pub fn generate_matrix(size: usize, n_walks: u32, mut notify_progress: impl FnMut(u32, u32)) -> RankMatrix {
    // Define constants and algorithms parameters
    let neighbour_coords = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let vicinity_ratio = 2;
    let escape_ratio = 3;

    // Initialize
    let mut rg_pixels = rand::rngs::SmallRng::from_entropy();
    let mut rg_neighbours = rand::rngs::SmallRng::from_entropy();
    let mut bool_matrix = vec![vec![true; size]; size];
    let mut rank_matrix = vec![vec![0 as u32; size]; size];
    let center = (size + 1) / 2;
    // Minimal cluster radius considered
    let mut r_cluster = 50;
    bool_matrix[center][center] = false;

    for n in 0..n_walks {
        // Update boxes limits
        let r_vicinity = vicinity_ratio * r_cluster;
        let r_escape = escape_ratio * r_cluster;
        let cluster_vicinity = Box::from_center_radius(size, center, r_vicinity);
        let cluster_horizon = Box::from_center_radius(size, center, r_escape);

        // Perform random walk
        let mut p = bool_matrix.sample_free_point(&mut rg_pixels, &cluster_vicinity);
        while bool_matrix.is_isolated(&p) {
            // Move the particle
            let displacement = neighbour_coords.choose(&mut rg_neighbours).unwrap();
            p = Point {
                x: (p.x as i32 + displacement.0) as usize,
                y: (p.y as i32 + displacement.1) as usize,
            };

            // Restart when the particle escapes
            if !cluster_horizon.contains(&p) {
                p = bool_matrix.sample_free_point(&mut rg_pixels, &cluster_vicinity);
            }
        }
        bool_matrix[p.x][p.y] = false;
        rank_matrix[p.x][p.y] = n + 1;
        r_cluster = update_cluster_radius(r_cluster, center, &p);

        notify_progress(n, n_walks);
    }
    return rank_matrix;
}
