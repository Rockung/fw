function sum( [ num1, num2 = 0 ] ) {
    console.log( num1 + num2 );
}
sum( [ 1, 2 ] );
sum( [ 1 ] );
sum( [  ] );
// 3 // 1 // NaN