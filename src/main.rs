// use std::fs::File;
use std::fs;

// use std::io::Read;
use std::path::Path;
fn main() {
    // opening a file
    let file_path = Path::new("src").join("hello.txt");
    // let mut txt_file = File::open(file_path).expect("Failed to open file");

    //reading the file
    let content = fs::read_to_string(file_path).expect("Failed to read file content");
    println!("{}", content); 

   
}
