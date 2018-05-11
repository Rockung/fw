// Early returns: a combination of match statements and early returns

use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str)
        -> AliasedResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e)           => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e)            => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
