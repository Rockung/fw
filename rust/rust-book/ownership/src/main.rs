fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    let mut s = String::from("Hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first index: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
