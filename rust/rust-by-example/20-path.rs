// Path: posix::Path, windows::Path
//   a vector of bytes, not an UTF-8 string, 
//   use to_str() to convert

use std::path::Path;

fn main() {
    let path = Path::new(".");

    let _display = path.display();

    // merge a path using OS-specific separator
    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None    => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}