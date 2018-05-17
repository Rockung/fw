// std::env::args: command line arguments
//    returns an iterator that yields a String for each argument

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0]: the path to call the program
    println!("My path is {}.", args[0]);

    // command line parameters
    println!("I got {:?} arguments: {:?}.", args.len()-1, &args[1..]);
}