var item = {
    name: "Apples",
    quantity: 5
};
var { name = "Oranges", quantity = 3, value = 25 } = item;
console.log(name);
console.log(quantity);
console.log(value);
// "Apples"
// 5
// 25