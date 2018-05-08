// explicit lifetime annotation
//    * borrow checker uses it to determine how long refs should be valid
//    * apostrophe form      : foo<'a>
//    * multiple lifetimes   : foo<'a, 'b>
//    * applied to a ref type: &'a T1, &'b T2

// the two refs have different lifetimes which must both be at least
// as long as the function
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x;
    // the lifetime of &_x is shorter than that of 'y'.
    // a short lifetime cannot be coerced into a longer one.
}

fn main() {
    let (four, nine) = (4, 9);

    // any input which is borrowed must outlive the borrower
    print_refs(&four, &nine);

    failed_borrow();
}