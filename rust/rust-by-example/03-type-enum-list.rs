use List::*; // forward reference

// linked list
enum List {
    // wrap an element and a pointer to the next node
    Cons(u32, Box<List>),
    // signify the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    // insert into the front of the linked list
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // take a reference to the tail instead of the ownership
            // of the tail because 'self' is borrowed
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
