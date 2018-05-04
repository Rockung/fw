fn main() {
    // a..b  yields values from a(inclusive) to b(exclusive) in steps of one
    // a..=b can be used for a range that is inclusive on both ends
    
    for n in 1..21 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("fizz");
        } else {
            println!("{}", n);
        }
    }


}
