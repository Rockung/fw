// #[derive]: provide basic implementations for some traits
//    * comparison traits: Eq, PartialEq, Ord, PartialOrd
//    * Clone, to create T from &T via a copy
//    * Copy, copy semantics instead of move semantics
//    * Hash, to compute a hash from &T
//    * Default, to create an empty instance of a data type
//    * Debug, to format a value using the {:?} formatter

// a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// a tuple struct no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // it doesn't implement the 'Debug' trait
    // println!("One second looks like: {:?}", _one_second);

    // it doesn't implement the 'PartialEq' trait
    // let _this_is_true = (_one_second == _one_second);
    
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
           "bigger"
        };
    
    println!("One foot is {} than one meter.", cmp);
}