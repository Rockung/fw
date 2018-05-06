// the visibility of fields in a struct defaults to private
// can be overriden with the pub modifier

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // their fields can be normally accessed
    println!("The open box contains: {}", open_box.contents);

    //let closed_box = my::ClosedBox { contents: "classified information" };
    let _closed_box = my::ClosedBox::new("classified information");

    //println!("The closed box contains: {}", _closed_box.contents);
}