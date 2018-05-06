fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo { x: (1, 2), y: 3 };

    // destructure members using variables
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    // the order is not important
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // ignore some variables
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // it's wrong when it's incomplete
    // let Foo { y } = foo;
}
