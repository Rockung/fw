// Associated types: improves the overall readability of code by moving
//    inner types locally into a trait as output types

struct Container(i32, i32);

// trait Contains<A, B>
// store types inside the container
trait Contains {
    // generic types which methods will be able to utilize
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // specify the types for the trait
    type A = i32;
    type B = i32;

    // &Self::A and &Self::B are also valid here
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self)  -> i32 { self.1 }
}

// difference<A, B, C>(container: &C)
// no need to use A,B becuase they are inside the trait
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2, 
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
