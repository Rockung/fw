// aliasing:
//    * immutably borrowed any number of times
//    * can't be mutably borrowed while immutably borrowed
//    * only one mutable borrow is allowed at a time

struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // accessed via the refs and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // can't borrow as mutable because it's currently borrowed
        // as immutable
        // let mutable_borrow = &mut point;
    }

    {
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // can't borrow as immutable because it's currently borrowed
        // as mutable
        // let y = &point.y;

        // can't be borrowed after mutably borrowed
        // println!("Point Z coordinate is {}", point.z);

        // mutable refs can be passed as immutable to 'println!'
        println!("Point has coordinates: ({}, {}, {})",
                    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
    }

    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             borrowed_point.x, borrowed_point.y, borrowed_point.z);
}