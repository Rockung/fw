// format!: write formatted text to String
// print!: same as format!, but the text is printed to the console(io::stdout).
// println!: same as print!, but a newline is appended
// eprint!: the text is printed to the standard error(io::stderr).
// eprintln!: same as eprint!, but a newline is appended

fn main() {
    // '{}' will be replaced with arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    
    // Special formatting after a ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // right-align and padding
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    // decimal point
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}