// Variables in Rust do more than just hold data in the stack: they also
// own resources, e.g. Box<T> owns memory in the heap.

// RAII: Resource Acquisition Is Initialization
//       whenerver an object goes out of scope, its destructor is called
//       and its owned resources are freed.
//       RAII shields against resource leak bugs, you'll never have to 
//       manually free memory or worry about memory leaks again!

fn create_box() {
    let _box1 = Box::new(3i32);
    // _box1 is destroyed here, and memory gets freed
}

fn main() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
        // _box3 is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // _box2 is destroyed here, and memory gets freed
}
