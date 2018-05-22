fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    let mut s = String::from("Hello world");

    // immutable borrow here
    let word = first_word(&s);

    // mutable borrow occurs here
    // s.clear();
    println!("the first index: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
