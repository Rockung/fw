var num1, num2, rest;
var x, y, z;
[num1, num2, ...rest] = [1, 2, 3, 4, 5];
[x, y, z] = [1, 2, 3, 4, 5];
console.log(num1);
console.log(num2);
console.log(rest);
console.log(x, y, z)
// 1
// 2
// [3, 4, 5]
// 1 2 3