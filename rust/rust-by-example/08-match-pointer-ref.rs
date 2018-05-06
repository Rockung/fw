// reference in Rust is very different from pointer in C
// destructuring and dereferencing are different concepts
// dereferencing uses *
// destructuring uses &, ref, and ref mut

fn main() {
    // & signifies there is a reference being assigned
    // let reference = &4;
    let ref reference = 4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // dereference before matching
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    let value = 5;
    match value {
        // create a reference
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        },
    }
}
