// borrowing: access data without taking ownership over it
//            pass objects by reference(&T) instead of by value(T)
// borrow checker: references always point to valid objects
//            the object cannot be destroyed while references to an object exist

// takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// borrow an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // take a reference to the data contained inside the box
        let _ref_to_i32_1: &i32 = &boxed_i32;
        let _ref_to_i32_2: &i32 = &boxed_i32;
        
        // can't destroy 'boxed_i32' while the inner value is borrowed
        // eat_box_i32(boxed_i32);
    }

    eat_box_i32(boxed_i32);
}