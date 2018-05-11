// define an error type: 
//    mask all of the different errors with a single type of error.
// In general, a "good" error type:
//    * represents different errors with the same type
//    * presents nice error messages to the user
//    * easy to compare with other types: Err(EmptyVec)
//    * hold information about the error: Err(BadChar(c, position))
//    * compose well with other errors

use std::error;
use std::fmt;

// DoubleError =====================================================
// define our error types which can be customized for our error handling
// cases
#[derive(Debug, Clone)]
struct DoubleError;

// no state info about the error. for example, which string failed to parse
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // generic error, underlying cause isn't tracked
        None
    }
}
// DoubleError =====================================================

type Result<T> = std::result::Result<T, DoubleError>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>()
            .map_err(|_| DoubleError)
            .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
