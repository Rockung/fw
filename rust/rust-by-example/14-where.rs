// Where clauses: more expressive than using normal syntax
//    * bounds to type parameters
//    * bounds to arbitrary types

// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// impl <A, D> MyTrait<A, D> for YourType where
//      A: TraitB + TraitC,
//      D: TraitE + TraitF {}

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// indirect approach: where clause
impl<T> PrintInOption for T where Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
