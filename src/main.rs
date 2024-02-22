use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
fn main(){
    // opening a file in read mode
    let file_path = Path::new("src").join("How to use FFMPEG.mp4");
    let mut mp4_file = File::open(file_path).expect("Failed to open file");

    //reading file into Bufreader 
    let mut mp4_reader = BufReader::new(&mut mp4_file);
    
    let mut data = Vec::new();
    mp4_reader.read_to_end(&mut data).expect("Faileed to read file");

    println!("{:?}",data); 
}
