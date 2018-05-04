// A rust program is (mostly) made up of a series of statements
// The most common kinds of statements
//     declare a variable binding
//     use a ; with an expression

// Blocks are expressions, which can be used as r-values in assignments
// The last expression in the block will be assigned to the l-value.
// If the last expression of the block ends with a semicolon, the return
// value will be ().

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}