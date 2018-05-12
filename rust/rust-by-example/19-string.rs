// Strings
//    * String: std::string
//         * stored as vector of bytes(Vec<u8>)
//         * valid UTF-8 sequence
//         * heap-allocated
//         * not null terminated
//    * &str: std::str
//         * a slice(&[u8]) points to a valid UTF-8 sequence
//         * can be used to view into a String

fn main() {
    // a ref to a string allocated in read only memory
    let pangram = "the quick brown fox jumps over the lazy dog";
    // let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // create an empty and growable String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("Sorted string: {}", string);

    // the trimmed string is a slice to the original string,
    // hence no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // heap-allocate a string
    let alice = String::from("I like dogs");
    // allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}