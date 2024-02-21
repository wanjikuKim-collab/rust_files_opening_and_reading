use std::fs::File;

use std::io::Read;
use std::path::Path;
fn main() {
    // opening a file
    let file_path = Path::new("src").join("hello.txt");
    let mut txt_file = File::open(file_path).expect("Failed to open file");

    // match file{
    //     Ok(_file) => {
    //         println!("file successfully opened!");
    //     }
    //     Err(error) => {
    //         println!("Failed to open file: {:?}", error);
    //     }
    // }

   
}
