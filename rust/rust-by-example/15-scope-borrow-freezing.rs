// freezing: when data is immutably borrowed
//           frozen data can't be modified vai the original object

fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _large_integer = &_mutable_integer;
        
        // frozen in its scope
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}