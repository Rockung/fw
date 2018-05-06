// the compiler implicitly creates a new anonymous structure to store the
// captured variables inside, meanwhile implementing the functionality via
// one of the traits: Fn, FnMut, or FnOnce for this unknown type.

fn apply<F>(f: F) where F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture 'x' into an anonymous type and implement 'Fn' for it.
    // Store it in 'print'
    let print = || println!("{}", x);
    apply(print);
}