// Trait: a collection of methods defined for an unknown type: Self
//        They can access other methods declared in the same trait

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // static method signature: Self refers to the implementor type
    fn new(name: &'static str) -> Self;

    // instance method signature: &self
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // implementor methods can use the implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    // Self is the implementor type: Sheep
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaaah?"
        } else {
            "baaaaaaah!"
        }
    }

    // default trait methods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // type annotation is necessary in this case
    // let mut dolly: Sheep = Animal::new("Dolly");
    let mut dolly: Sheep = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
