// non-copyable types
struct Empty;
struct Null;

// generic trait over T
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// T: generic parameter
// U: caller
impl <T, U> DoubleDrop<T> for U {
    // take ownership of both passed parameters
    // deallocate both
    fn double_drop(self, _: T) {
        println!("something here!");
    }
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // deallocate empty and null
    empty.double_drop(null);

    // empty;
    // null;
}