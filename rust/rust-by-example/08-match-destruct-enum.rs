#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    use Color::*;

    let color = RGB(122, 17, 40);
    println!("What color is it?");

    match color {
        Red   => println!("The color is Red!"),
        Blue  => println!("The color is Blue!"),
        Green => println!("The color is Green!"),
        RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        // Don't need another arm because all variants have been examined
    }    
}
