// lifetime of a variable
//    * a contruct the compiler uses to ensure all borrows are valid
//    * begins when it is created and ends when it is destroyed
// lifetime vs scope
//    they are often referred to together, but they are not the same

fn main() {
    let i = 3; // lifetime for 'i' starts
    
    {
        let borrow1 = &i; // 'borrow1' lifetime starts
        println!("borrow1: {}", borrow1);
    } // 'borrow1' ends

    {
        let borrow2 = &i; // 'borrow2' lifetime starts
        println!("borrow2: {}", borrow2);
    } // borrow2 ends

} // 'i' ends