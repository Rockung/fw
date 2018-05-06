fn main() {
    // type Option<i32>
    let mut optional = Some(0);

    // destructure 'optional' into 'Some(i)', evaluate the block, else break
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            // let's break the while
            optional = None;
        } else {
            println!("'i' is '{:?}'. Try again.", i);
            optional = Some(i + 1);
        }
    }
}