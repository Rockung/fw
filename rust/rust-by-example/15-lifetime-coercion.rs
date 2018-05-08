// Coercion: a longer lifetime can be coerced into a shorter one

// Rust infers a lifetime that is as short as possible
fn multipy<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// 'a: 'b read as lifetime 'a is at least as long as 'b
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // longer lifetime

    {
        let second = 3; // shorter lifetime
        println!("The product is {}", multipy(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}