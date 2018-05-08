// 'static lifetime
//    * lasts for lifetime of the running program
//    * may be coerced to a shorter lifetime
//    * stored in the read-only memory of the binary
// two ways to make a variable with 'static lifetime
//    * make a constant with the static declaration
//    * make a string literal which has type: &'static str

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn coerce_static_no_arg<'a>() -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // when static_string goes out of scope, the reference
        // can no longer be used, but the data remains in the binary
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("coerce_static_no_arg: {} ", coerce_static_no_arg());
    println!("NUM: {} stays accessible!", NUM);
}