// Clone: when dealing with resources
//    * default behavior: transfer them during assignments or function calls
//    * make a copy sometimes as well by implementing Clone trait

// without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// a tuple struct with resources that implements Clone
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;

    // copy, there are no resources to move
    let copied_nil = nil;

    // both can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // copy pair into moved_pair, moves resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // 'pair' has lost its resources
    // println!("original: {:?}", pair);

    // clone moved_pair into cloned_pair(including resources)
    let cloned_pair = moved_pair.clone();

    // drop the original pair using std::mem::drop
    drop(moved_pair);

    // println!("copy: {:?}", moved_pair);

    // the result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}