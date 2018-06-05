// Box<T>
//    * smart pointers, which implement Deref trait, are treated like references
//    * heap data is cleaned up because of Drop trait implementation

// Using Box<T> to get a recursive type with a known size
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, 
                Box::new(Cons(2, 
                Box::new(Cons(3, 
                Box::new(Nil))))));
}
