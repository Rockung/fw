var [ x, y, z ] = ['a', 'b', 'c'];
var { a: a, b: b, c: c } = {a: 1, b: 2, c: 3};
console.log( x, y, z );             // a b c
console.log( a, b, c );             // 1 2 3

// Breaking the data structure into smaller parts using 
// destructuring makes fetching the data you need much easier. 

// Destructuring in ES6 uses the object and array literal notation
// syntax, which traditional users of JavaScript are already familiar with.
