var product = (x, y) => x * y;
console.log(product.call(null, 2, 3));
console.log(product.apply(null, [2, 3]));
var multiply = product.bind(null, 2, 3);
console.log(multiply ());
