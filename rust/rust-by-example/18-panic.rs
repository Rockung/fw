// panic
//    * the simplest error handling mechanism
//    * prints an error message
//    * starts unwinding the task
//    * exits the program

fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
