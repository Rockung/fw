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
let { apple: { quantity }} = items;
console.log(quantity);        // 5