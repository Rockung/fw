fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v = vec![1, 2, 3, 4, 5];
    
    for i in &mut v {
        *i += 50;
    }

    let third: &i32 = &v[2];
    println!("{}", third);

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    for i in &v {
        println!("{}", i);
    }
}