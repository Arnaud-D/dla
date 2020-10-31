pub struct Cli {
    pub output_filename: String,
    pub img_size: usize,
    pub n_walks: u32,
    pub mode: u32,
}

impl Cli {
    pub fn parse_args() -> Result<Cli, String> {
        let output_filename = match Cli::parse_filename() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let img_size = match Cli::parse_img_size() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let n_walks = match Cli::parse_n_walks() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let mode = match Cli::parse_mode() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        return Ok(Cli { output_filename, img_size, n_walks, mode});
    }

    fn parse_filename() -> Result<String, String> {
        return match std::env::args().nth(1) {
            Some(a) => Ok(a),
            None => Err("No output filename provided.".to_string())
        }
    }

    fn parse_img_size() -> Result<usize, String> {
        return match std::env::args().nth(2) {
            Some(img_size_str) => {
                match img_size_str.parse::<usize>() {
                    Ok(img_size) => Ok(img_size),
                    Err(_) => Err("Invalid image size provided.".to_string())
                }
            }
            None => Err("No image size provided.".to_string())
        };
    }

    fn parse_n_walks() -> Result<u32, String> {
        return match std::env::args().nth(3) {
            Some(n_walks) => {
                match n_walks.parse::<u32>() {
                    Ok(n_walks) => Ok(n_walks),
                    Err(_) => Err("Invalid number of particles provided.".to_string())
                }
            }
            None => Err("No number of particles provided.".to_string())
        };
    }

    fn parse_mode() -> Result<u32, String> {
        return match std::env::args().nth(4) {
            Some(mode) => {
                match mode.parse::<u32>() {
                    Ok(mode) => {
                        if mode == 0 || mode == 1 {
                            Ok(mode)
                        } else {
                            Err( format!("Mode `{}` does not exist. Available modes are `0` or `1`.", mode))
                        }
                    },
                    Err(_) => Err("Invalid mode provided.".to_string())
                }
            },
            None => Ok(0)
        };
    }
}
