// Box, stack and heap
//    * all values are stack-allocated by default
//    * values can be allocated in the heap by creating a Box<T>
//    * a box is a smart pointer to a heap allocated value of T
//    * when a box goes out of scope
//         * its destructor is called
//         * the inner object is destroyed
//         * the memory in the heap is freed
//    * use * operator to dereference the boxed value

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // allocate in the heap, return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // stack-allocated
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // heap-allocated
    let boxed_rectangle = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    // the output of functions can be boxed
    let boxed_point = Box::new(origin());

    // double indirection
    let box_in_a_box = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));
}
