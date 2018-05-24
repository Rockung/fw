enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            &SpreadsheetCell::Int(x)      => println!("{}", x),
            &SpreadsheetCell::Float(f)    => println!("{}", f),
            &SpreadsheetCell::Text(ref s) => println!("{}", s),
        }
    }
}
