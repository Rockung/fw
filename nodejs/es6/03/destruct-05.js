let item = {
        name: "Apples",
        quantity: 5
    },
    name = "Oranges",
    quantity = 3;
// assigning different values using destructuring
({ name, quantity } = item);
console.log(name);          // "Apples"
console.log(quantity);      // 5
