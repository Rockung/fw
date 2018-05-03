// hide warnings for unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly 'use' each name without manual scoping
    use Status::{ Poor, Rich };
    // automatically 'use' each name inside 'Work'
    use Work::*;

    // equivalent to 'Status::Poor'
    let status = Poor;
    // equivalent to 'Work::Civilian'
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money ..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
