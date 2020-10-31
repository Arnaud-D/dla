mod matrix;
mod image;

struct Progress {
    n_skips: u32,
    skip: u32,
}

impl Progress {
    fn new(n_skips: u32, skip: u32) -> Progress {
        Progress { n_skips, skip }
    }

    fn display(&mut self, n: u32, n_walks: u32) -> () {
        if self.skip < self.n_skips {
            *self = Progress::new(self.n_skips, self.skip + 1)
        } else {
            println!("Random walk {}/{}.", n + 1, n_walks);
            *self = Progress::new(self.n_skips, 1)
        }
    }
}

struct Cli {
    output_filename: String,
    img_size: usize,
    n_walks: u32,
}

impl Cli {
    fn parse_args() -> Cli {
        let output_filename = std::env::args().nth(1).expect("no filename given");
        let img_size = std::env::args().nth(2).expect("no image size given");
        let n_walks = std::env::args().nth(3).expect("no number of walks given");
        return Cli {
            output_filename,
            img_size: img_size.parse::<usize>().unwrap(),
            n_walks: n_walks.parse::<u32>().unwrap()
        };
    }
}


fn main() {
    let args = Cli::parse_args();

    // Record start time
    let start_time = std::time::SystemTime::now();

    // Generate image
    let n_skips = 150;
    let mut progress = Progress::new(n_skips, n_skips - (args.n_walks - 1) % n_skips);
    let display = |n: u32, n_walks: u32| -> () { progress.display(n, n_walks) };
    let matrix = matrix::generate_matrix(args.img_size, args.n_walks, display);
    let img = image::convert_matrix(matrix);

    // Display elapsed time
    let elapsed = start_time.elapsed().unwrap();
    println!("Executed in {} seconds.", elapsed.as_secs());

    // Save output image
    match img.save(&args.output_filename) {
        Ok(_) => println!("File `{}` saved successfully.", args.output_filename),
        Err(_) => println!("An error occured when saving the image. The file `{}` might be write-protected.", args.output_filename),
    }
}
