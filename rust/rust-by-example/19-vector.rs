// Vector: std::vec
//    * resizable arrays: grow, shrink
//    * represented: a pointer to the data, its length, and its capacity
//    * grows as long as the length is smaller than the capacity

fn main() {
    // iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into {:?}", collected_iterator);

    // vec! macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // immutable vectors can't grow
    // collected_iterator.push(0);

    println!("Vector size: {}", xs.len());        // length
    println!("Second element: {}", xs[1]);        // indexing square brackets
    println!("Pop last element: {:?}", xs.pop()); // remove tha last element
    // println!("Fourth element: {}", xs[3]);     // out of bounds indexing

    // iteration
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // enumeration: index and content
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // mutable iteration
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}