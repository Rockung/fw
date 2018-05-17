use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // path to the file
    let path = Path::new("hello.txt");
    let display = path.display();

    // return io::Result<File>
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // read the file contents into a string,
    // return io::Result<usize>
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(_)    => println!("{} contains:\n{}", display, s),
    }
}