mod matrix;
mod image;

fn main() {
    let output_filename = "test.png";
    let img_size = 501;
    let n_walks = 20000;

    let start_time = std::time::SystemTime::now();

    let matrix = matrix::generate_matrix(img_size, n_walks);
    let img = image::convert_matrix(matrix);

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
