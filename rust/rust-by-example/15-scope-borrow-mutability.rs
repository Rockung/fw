// &mut T: mutably borrowed, mutable reference
//         gives read/write access to the borrower
//     &T: immutable reference
//         the borrower can read the data but not modify it

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // &'static str is a reference to a string allocated in read-only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "Dauglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };
   
    // create a mutable copy
    let mut mutabook = immutabook;

    // immutable borrow an immutable object
    borrow_book(&immutabook);

    // immutable borrow an mutable object
    borrow_book(&mutabook);

    // borrow a mutable object as mutable
    new_edition(&mut mutabook);

    // borrow an immutabe object as mutable
    // new_edition(&mut immutabook);
}