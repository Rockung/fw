# Variables and Mutability

* variables are immutable by default
* let mut x = 5;

## Differences Between Variables and Contants

* contants are always immutable
* declared by 'const' instead of 'let'
* type of the value must be annotated
* set only to a constant expression
* const MAX_POINTS: u32 = 100_000;

> constant naming convention: uppercase with underscores between words
> naming hardcoded values makes your code easy to maintain

## Shadowing
> declare a new variable with the same name as a previous variable, so the first variable is shadowed by the second.

* let x = 5;
* let x = x + 1;
* let x = x * 2;

# Data Types
> every value is of a certain data type, which tells Rust

  * what kind of data is
  * how to work with that data

> two data type subsets: scalar and compound

> statically typed language: it must know the types of all variables at compile time

## Scalar Types

* integer       : i8/u8, i16/u16, i32/u32, i64/u64, isize/usize
* floating-point: f32, f64
* boolean       : true, false
* character     : char

## Compound Types

* tuple: group a variety of types together
* array: a collection of values with the same type

## Integer Literals
* decimal : 98_222
* hex     : 0xfe
* octal   : 0o77
* binary  : 0b1111_0000
* byte(u8): b'A'

## Numeric Operations
* addition      : +
* subtraction   : -
* multiplication: *
* division      : /
* remainder     : %

## Tuple Member Accessing
* destructing by pattern matching
    - let tup = (500, 6.4, 1);
    - let (x, y, z) = tup;
* by indexing
    - let five_hundred   = tup.0
    - let six_point_four = x.1;

# Control Flow
* deciding whether or not to run some code
* deciding to run some code repeatedly

## if Expressions
## loops: loop, while, for