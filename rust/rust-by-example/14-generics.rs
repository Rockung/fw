// Generics: generalizing types and functionalities to broader cases
//    * reduce code duplication
//    * type parameter  : <Aaa, Bbb, ...>, upper camel case
//    * generic type    : Array<T>
//    * generic function: fn foo<T>(arg: T) { ... }

// A: concrete type
struct A;

// Single: concrete type
struct Single(A);

// SingleGen: generic type
//         T: type parameter
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    // instantiation of generic type bying specifying type parameter
    let _char: SingleGen<char> = SingleGen('a');

    // implicitly specify type parameter
    let _t    = SingleGen(A);   // SingleGen<A>
    let _i32  = SingleGen(6);   // SingleGen<i32>
    let _char = SingleGen('a'); // SingleGen<char>
}