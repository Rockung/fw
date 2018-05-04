// type statement can be used to give a new name to an existing type
// types must have CamelCase names. the exception to this rule are 
// the primitive types: usize, f32, etc.

// The main use of aliases is to reduce boilerplate; for example, 
// IoResult<T> is an alias for Result<T, IoError>

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // type aliases *don't* provide any extra type safety,
    // because aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}