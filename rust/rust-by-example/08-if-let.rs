fn main() {
    let number = Some(7);
    // if 'let' destructures 'number' into 'Some(i)',
    // evaluate the block '{}'
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let emoticon: Option<i32> = None;
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
    
    let a = Foo::Bar;
    // Variable 'a' matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    let b = Foo::Baz;
    if let Foo::Bar = b {
        println!("b is foobar")
    } else {
        println!("b is not foobar")
    }

    let c = Foo::Qux(100);
    // Variable 'c' matches Foo::Qux which has a value,
    // and destructure the value into variable 'value'
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}

#[allow(dead_code)]
enum Foo {
    Bar,
    Baz,
    Qux(i32),
}