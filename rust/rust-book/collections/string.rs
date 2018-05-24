fn main() {
    // let mut s = String::new();

    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('l');
    println!("s2 is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = " ...";

    // let s = s1 + "-" + &s2 + "-" + &s3 + s4;
    // let s = s1 + "-" + &s2 + "-" + &s3 + &s4;
    let s = format!("{}-{}-{}-{}", s1, s2, s3, s4);
    println!("{}", s);

    let hello = "Здравствуйте";
    println!("{}", hello.len());
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}