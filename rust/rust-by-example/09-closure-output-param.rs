// returning closures types are problematic because Rust currently only
// supports returning concrete(non-generic) types.
// returning a closure is only possible by making it concrete via boxing

// The valid traits for returns are slightly different than before:
//     * Fn: normal
//     * FnMut: normal
//     * FnOnce: unstable, FnBox type is currently needed

// the *move* keyword must be used, which signals that all captures occur
// by value.

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    // let fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
