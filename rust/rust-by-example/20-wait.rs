use std::process::Command;

fn main() {
    // spawn return Option<Child>, so unwrap to process::Child
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    
    // process::ExitStatus, Option<i32>
    let result = child.wait().unwrap();

    println!("reached end of main: {}", result);
}