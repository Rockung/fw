## Borrowing and Lifetimes
> typical programming errors regarding borrowing
> how to prevent these errors using a borrow checker
> restrict the scope of borrowing
> lifetime specifiers for function's return

### Ownership and Borrowing
let n = 12;
let ref_to_n = &n;
> n, *ref_to_n refer to the same object, only n owns it
> ref_to_n can access the object, but it does not own it
> ref_to_n borrows the same number owned by n
> borrowing begins the ref begins to refer to the object, and ends when the ref is destroyed

let mut n = 12;
let ref1_to_n = &mut n;
let ref2_to_n = &n;
> ref1_to_n borrows mutably the number owned by n
> ref2_to_n borrows immutably the number owned by n

### Object Lifetimes
> scope: compile time, lifetime: runtime
> lifetime: between creating and destroying
> scope begins when the variable is declared, lifetime begins when the object receives a value

### Errors Regarding Borrowing
> use after drop: the borrowing var must be declared after the borrowed var
> use after change by an alias: in any point of the code, any object cannot have at the same time a mutable borrowing and some other borrowing




