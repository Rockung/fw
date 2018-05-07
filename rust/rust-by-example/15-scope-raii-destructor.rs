// Drop trait: the destructor in Rust which is called when the resource
// goes out of scope. Only implement it when requiring its own destructor
// logic.

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}