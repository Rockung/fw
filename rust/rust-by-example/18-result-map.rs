// Result: a richer version of the Option that describes possible
//         error instead of possible absence
// Result<T, E> have one of two outcomes:
//    * Ok<T>:  an element T was found
//    * Err<E>: an error was found with element E
// Like Option, Result has many methods associated with it. unwrap(),
// e.g., either yields the element T or panic S.

use std::num::ParseIntError;

fn multiply_v1(first_number_str: &str, second_number_str: &str)
        -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                     Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn multiply_v2(first_number_str: &str, second_number_str: &str)
        -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply_v1("10", "2");
    print(twenty);

    let tt = multiply_v2("t", "2");
    print(tt);
}