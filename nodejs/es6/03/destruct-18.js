function printNums( { num1, num2 } ) {
    console.log( num1, num2 );
}
printNums( { num2: 1, num1: 2 } );
printNums( { num2: 42 } );
// 2 1
// undefined 42