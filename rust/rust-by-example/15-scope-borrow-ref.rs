// ref pattern: take references to the fields of a struct/tuple
//    * pattern matching
//    * destructuring via let binding

#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // A 'ref' borrow on the left side of an assignment
    // is equivalent to an '&' borrow on the right side
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);


    let point = Point { x: 0, y: 0 };
    let _copy_of_x = {
        // ref_to_x is a ref to the x field of point
        let Point { x: ref ref_to_x, y: _ } = point;
        // return a copy of the x field of point
        *ref_to_x
    };

    // a mutable copy
    let mut mutable_point = point;
    {
        // ref mut: take mutable references
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (ref mut first, ref mut last) = mutable_tuple;
        *first = Box::new(6u32);
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}