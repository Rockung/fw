use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // spawn the 'wc' command
    let process = match Command::new("wc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
        Err(why)    => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // write a string to the stdin of wc
    // stdin has type Option<ChildStdin>, so unwrap it out
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_)    => println!("sent pangram to wc"),
    }

    // stdout has type Option<ChildStdout>
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_)    => print!("wc responded with:\n{}", s),
    }
}