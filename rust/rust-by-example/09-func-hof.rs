// HOF: Higher Order Functions which take one or more functions and/or
// produce a more useful function.
// HOFs and lazy iterators give Rust its functional flavor.

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
       
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                              // all natural numbers squared
             .take_while(|&n_squared| n_squared < upper)  // below upper limit
             .filter(|&n_squared| is_odd(n_squared))      // That are odd
             .fold(0, |acc, n_squared| acc + n_squared);  // sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}