use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Into trait is simply the reciproca of From trait. If you have 
// implemented From trait for your type, you get Into implementation
// for free.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // let num = int.into();
    let num: Number = int.into();
    println!("My number is {:?}", num);
}