fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];
    
    // iter borrows each element of the collection through each iteration
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter converts the collection into an iterator
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }    

    let mut names = vec!["Bob", "Frank", "Ferris"];

    // iter_mut mutably borrows each element of the collection, allowing for
    // the collection to be modified in place
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
