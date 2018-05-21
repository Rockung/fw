extern crate doctest;

use doctest::Person;

fn main() {
    let john = Person::new("John");

    john.hello();
}
