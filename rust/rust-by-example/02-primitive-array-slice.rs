// Array
// a collection of objects of the same type T, stored in contiguous
// memory. Array are created using brackets [], and their size, which
// is known at compile time, is part of their type signature [T; size]

// Slice
// similar to arrays, but their size is not know at compile time.
// A slice is a two-word object, the first word is a pointer to the
// data, and the second word is the length of the slice. The word size
// is the same as usize. Slices can be used to borrow a section of an
// array, and have the type signature &[T]

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array accupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // println!("{}", xs[5]);
}

