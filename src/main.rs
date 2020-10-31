mod matrix;
mod image;

struct Progress {
    n_skips:u32,
    skip:u32
}

impl Progress {
    fn new(n_skips:u32, skip:u32) -> Progress {
        Progress {n_skips, skip}
    }

    fn display(&mut self, n:u32, n_walks:u32) -> () {
        if self.skip < self.n_skips {
            *self = Progress::new(self.n_skips, self.skip + 1)
        } else {
            println!("Random walk {}/{}.", n + 1, n_walks);
            *self = Progress::new(self.n_skips, 1)
        }
    }
}


fn main() {
    let output_filename = "test.png";
    let img_size = 501;
    let n_walks = 10000;

    // Record start time
    let start_time = std::time::SystemTime::now();

    // Generate image
    let n_skips = 150;
    let mut progress = Progress::new(n_skips, n_skips - (n_walks-1) % n_skips);
    let display = |n:u32, n_walks:u32| -> () {progress.display(n, n_walks)};
    let matrix = matrix::generate_matrix(img_size, n_walks, display);
    let img = image::convert_matrix(matrix);

    // Display elapsed time
    let elapsed = start_time.elapsed().unwrap();
    println!("Executed in {} seconds.", elapsed.as_secs());

    // Save output image
    match img.save(output_filename) {
        Ok(_) => println!("File `{}` saved successfully.", output_filename),
        Err(_) => println!("An error occured when saving the image. The file `{}` might be write-protected.", output_filename),
    }
}
