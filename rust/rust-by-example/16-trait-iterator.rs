// Iterator: implement iterators over collections such as arrays
//    * only next method to be implemented
//    * automatically defined as in arrays and ranges
//    * .into_iterator turns collection into iterators in for contruct

struct Fibonacci {
    curr: u32,
    next: u32,
}

// next method for Iterator trait
impl Iterator for Fibonacci {
    type Item = u32;

    // define the sequence using '.curr' and '.next'
    // return type is Option<T>:
    //    * when the Iterator is finished, None is returned
    //    * Otherwise, the next value is wrapped in Some and returned
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence,
        // the Iterator will never return None, and Some is 
        // always returned
        Some(self.curr)
    }
}

// returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // 0..3 is an Iterator that generates: 0, 1, and 2
    let mut sequence = 0..3;

    println!("Four consecutive 'next' calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // 'for' works through an Iterator until it returns None
    // each Some value is unwrapped and bound to a variable i
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // take(n) reduces an Iterator to its first n terms
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // skip(n) shortens an Iterator by dropping its first n terms
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    // iter() produces an Iterator over an array/slice
    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}