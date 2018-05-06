// on-the-fly closures has no type annotation.
// type annotation is needed when a closure works as an input parameter
//      * Fn:     the closure captures by reference(&T)
//      * FnMut:  the closure captures by mutable reference(&mut T)
//      * FnOnce: the closure captures by value(T)

fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // non-copy type
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: 'greeting' by ref and 'farewell' by value
    let diary = || {
        // requires 'Fn'
        println!("I said {}.", greeting);

        // requires 'FnMut'
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // requires 'FnOnce'
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
