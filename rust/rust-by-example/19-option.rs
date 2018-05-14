// Option enum: catch the failure instead of calling panic!
//    * None, to indicate failure or lack of value
//    * Some(value), a tuple struct that wraps a value with type T

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    //  pattern matched
    match checked_division(dividend, divisor) {
        None           => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // binding 'None' to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);

    // unwrap() extracts the value wrapped
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // unwrap a 'None' will panic!
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}