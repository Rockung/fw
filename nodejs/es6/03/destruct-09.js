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
let { apple: {} } = items;
console.log(apple);      // ReferenceError: apple is not defined

// This destructuring statement has no bindings and because of the curly
// braces on the right, apple is not a variable binding to create but 
// rather is used as a location to inspect inside the items object. 
