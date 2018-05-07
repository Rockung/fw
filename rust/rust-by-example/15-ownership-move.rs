// * variables are in charge of freeing their own resource
// * resources can only have one owner which
// * resources cannot be freed more than once
// * not all variables own resources(e.g. references)

// move: the ownership of the resources is transferred
//    * assignments(let x = y)
//    * pass function arguments by value(foo(x))
// ownership moving avoids dangling pointers

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn main() {
    // allocated on stack
    let x = 5u32;

    // *copy* x into y - no resources are moved
    let y = x;

    // both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // 'a' is a pointer to a heap-allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // *move* 'a' into 'b'
    let b = a;
    // 'a' and 'b' are pointers to the same heap-allocated data,
    // but 'b' now owns it

    // println!("a contains: {}", a);

    // this func takes ownership of the heap-allocated memory from 'b'
    destroy_box(b);

    // since the heap memory has been freed at this point, this action
    // would result in dereferencing freed memory, but it's forbidden
    // by the compiler
    // println!("b contains: {}", b);
}
