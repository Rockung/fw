// Introducing '?': exactly equivalent to an unwrap which return S
//        instead of panic S on Err s

use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str)
        -> AliasedResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
