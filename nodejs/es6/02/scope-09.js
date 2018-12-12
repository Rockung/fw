// TDZ: Temporal Dead Zone, you are accessing a variable that's been 
//      declared but not yet initialized.
let data = true;
if (true) { // enter new scope, TDZ starts
    // Uninitialized binding for "data" is created
    console.log(data); // ReferenceError
    let data; // TDZ ends, "data" is initialized with "undefined"
}
console.log(data); // true
