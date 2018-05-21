// Unit Clarification
//    unit conversion: implement Add with a phantom type parameter

use std::ops::Add;
use std::marker::PhantomData;

// unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// length for unit types with phantom type parameter Unit
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// Add trait defines the behavior of operator '+'
impl <Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in ", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // nonsensical operations fail: type mismatch at compile time
    // let one_feter = one_foot + one_meter;
}