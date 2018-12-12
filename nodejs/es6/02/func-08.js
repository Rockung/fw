let myObj = {
    name: 'fancy',
    operation: function() {
        console.log(this);
} }
myObj.operation(); // { name: 'fancy', operation: [Function: operation]}

let x = myObj.operation;
x(); // Window

x.call(myObj); // { name: 'fancy', operation: [function]}
