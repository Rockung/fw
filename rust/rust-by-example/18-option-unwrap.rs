// Option & unwrap

// Option<T> in the std lib is used when absence is a possibility
//    * Some(T): an element of type T was found
//    * None: no element was found

// explicitly handled via match
fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(what)   => println!("{}? How nice.", what),
        None          => println!("No gift? Oh well."),
    }
}

// implicitly handled via unwrap
fn give_princess(gift: Option<&str>) {
    // return a panic when it receives a None
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food = Some("cabbage");
    let drink = Some("Coko cola");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(drink);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    // give_princess(snake);
    give_princess(nothing);
}