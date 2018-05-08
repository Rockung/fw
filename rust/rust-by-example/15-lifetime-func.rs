// function signatures with lifetimes(igoring elision)
//    * any reference must have an annotated lifetime
//    * any reference being returned must have the same 
//      lifetime as an input or be static
// Note: returning references without input is banned.

fn print_one<'a>(x: &'a i32) {
    println!("'print_one': x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// &String::from("foo") would create a string , followed by a reference.
// the data is dropped upon exiting the scope, leaving a reference to
// invalid data to be returned

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}