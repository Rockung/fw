// three types of structures
// 1. tuple structs, which are, basically, named tuples
// 2. unit structs, which are field-less, are useful for generics
// 3. classic C structs

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// unit struct
struct Nil;

// tuple struct
struct Pair(i32, f32);

// named tuples with two fields
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Peter";
    let age = 27;
    // let peter = Person { name, age };
    let peter = Person {name: name, age: age };
    println!("{:?}", peter);
    
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // struct update syntax to init other fields
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // 'let' binding destruction
    let Point { x: my_x, y: my_y } = point;

    // struct instantiation with an expression
    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    println!("The area of the rectangle: {}", rect_area(_rectangle));

    let _nil = Nil;

    // init a tuple struct and access the fields of the struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 }
    } = rect;

    return (x2 - x1) * (y2 - y1);
}
