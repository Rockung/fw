// Closures, also called lambda expressions or lambdas, are functions that
// can capture the enclosing environment. Calling a closure is exactly like
// calling a function

// on-the-fly usage: |val| val + x

fn main() {
    let closure_annotated = |i: i32| -> i32 { i+1 };
    let closure_inferred  = |i     |          i+1  ;

    let i = 1;
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred:  {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}