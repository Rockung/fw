var getChars = () => "abc";
var getNumbers = () => ({a: 1, b: 2, c: 3}); 

var a, b, c, x, y, z;
[x, y ,z] = getChars();
( { a, b, c } = getNumbers() );
console.log( x, y, z );
console.log( a, b, c );
// a b c
// 1 2 3

// You must always use parentheses around an object destructuring assignment
// statement. this is because an opening curly brace is used to denote the 
// start of a block statement. the parentheses around it denote that the curly 
// brace should be interpreted as an expression used in an assignment statement 
// using destructuring.
