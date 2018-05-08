// Bounds: like generic types can be bounded, lifetimes(themselves generic)
//         use bounds as well
//    * T: 'a: all refs in T must outlive lifetime 'a
//    * T: Trait + 'a: T implements Trait, refs in T outlive 'a

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where T: Debug {
    println!("'print': t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("'print_ref': t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
