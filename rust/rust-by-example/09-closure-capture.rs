// Closures can capture variables:
//     * by reference: &T
//     * by mutable reference: &mut T
//     * by value: T
// They preferentially capture variables by reference and only go lower when
// required.

fn main() {
    use std::mem;

    let color = "green";

    // 'println!' only requires 'by reference'
    let print = || println!("'color': {}", color);

    // Call the closure using the borrow
    print();
    print();

    // let _reborrow = &color;

    let mut count = 0;

    // '&mut count' is preferrable to 'count'
    let mut inc = || {
        count += 1;
        println!("'count': {}", count);
    };

    inc();
    inc();

    // let reborrow = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // 'mem::drop' requires 'T', so this must take by value.
    // A copy type would copy into the closure leaving the original untouched
    // A non-copy must move into the closure
    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };

    // this can only be called once.
    consume();
    // consume();

    // 'Vec' has non-copy semantics
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Removing 'move' from closure's signature will cause closure
    // to borrow _haystack_ variable immutably
    // println!("There're {} elements in vec", haystack.len());
}