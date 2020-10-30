use image;
use rand;
use rand::{Rng, SeedableRng};

fn main() {
    let output_filename = "test.png";
    let img_size = 501;
    let n_walks = 1000;

    let start_time = std::time::SystemTime::now();

    let mut img = image::GrayImage::new(img_size, img_size);
    let midpoint = (img_size + 1) / 2;
    let hot_pixel = image::Luma([255]);

    img.put_pixel(midpoint, midpoint, hot_pixel);
    let mut rg_pixels = rand::rngs::SmallRng::from_entropy();
    let mut rg_neighbours = rand::rngs::SmallRng::from_entropy();
    let mut x:i32;
    let mut y:i32;
    let dx: [i32; 4] = [1, -1, 0, 0];
    let dy: [i32; 4] = [0, 0, 1, -1];
    for n in 0..n_walks {
        x = rg_pixels.gen_range(1, img_size - 2) as i32;
        y = rg_pixels.gen_range(1, img_size - 2) as i32;
        while !(img.get_pixel((x + 1) as u32, y as u32) == &hot_pixel
            || img.get_pixel((x - 1) as u32, y as u32) == &hot_pixel
            || img.get_pixel(x as u32, (y + 1) as u32) == &hot_pixel
            || img.get_pixel(x as u32, (y - 1) as u32) == &hot_pixel
        ) {
            let neighbour_idx = rg_neighbours.gen_range(0, 4);

            x = x + dx[neighbour_idx];
            if x > 499 {
                x = 499;
            } else if x < 1 {
                x = 1;
            }

            y = y + dy[neighbour_idx];
            if y > 499 {
                y = 499;
            } else if y < 1 {
                y = 1;
            }
        }
        img.put_pixel(x as u32, y as u32, hot_pixel);
        println!("Random walk {}/{}.", n + 1, n_walks)
    }

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
