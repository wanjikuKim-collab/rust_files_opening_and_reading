use std::fs::File;
use std::path::Path;
use std::io::Read;
fn main() {
    // opening a file
    let file_path = Path::new("src").join("hello.txt");
    let mut txt_file = File::open(file_path).expect("Failed to open file");

    //reading the file in vector
    let mut contents = Vec::new();
    txt_file.read_to_end(&mut contents).expect("Failed to open file");
    println!("{:?}",contents); 
}
