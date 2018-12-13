function foo({ num1 = 42 } = {}, { num2 } = { num2: 42 }) {
    console.log( num1, num2 );
}
foo();          // 42 42
foo( {}, {} );  // 42 undefined

// destructuring default value and a function parameter default value