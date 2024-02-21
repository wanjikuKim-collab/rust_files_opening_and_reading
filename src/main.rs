use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // opening a file
    let file_path = Path::new("src").join("hello.txt");
    let txt_file = File::open(file_path).expect("Failed to open file");

    //reading the file in a buffer
    let reader = BufReader::new(txt_file);
    for line in reader.lines(){
        let line = line;
        println!("{:?}", line);
    }
}
