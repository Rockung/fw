var { a: foo, b: bar, c: baz } = {a: 1, b: 2, c: 3};
console.log( foo, bar, baz );         // 1 2 3
console.log( a, b, c );               // ReferenceError
