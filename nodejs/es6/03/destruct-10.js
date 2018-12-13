let items = {
    count : 2,
    name: "fruits",
    apple: {
        quantity: 5,
        value: 25
    },
    orange: {
        quantity: 3,
        value: 5
    }
};
let { apple = {} } = items;
console.log(apple);      // {quantity: 5, value: 25}
