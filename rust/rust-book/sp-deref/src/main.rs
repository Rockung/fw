use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // *y == *(y.deref())
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello("Rust");

    let m = MyBox::new(String::from("Rust"));
                
    // deref coercion:
    //               Deref      Deref 
    //                *m        &[..]
    // &MyBox<String> -> &String -> &str
    hello(&m);
    hello(&(*m)[..]);
}
