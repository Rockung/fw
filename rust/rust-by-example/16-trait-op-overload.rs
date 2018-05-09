// Operator overloading
//     * accomplish different tasks based on input arguments
//     * operators are syntactic sugar for method calls
//       e.g: a+b == a.add(b), add is part of Add trait

// Add can be found in core::ops
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// +: std::ops::Add, the trait for addition
// Add<Bar>: the trait for addition with a RHS of type Bar
// Foo + Bar = FooBar => foo.add(bar)
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
