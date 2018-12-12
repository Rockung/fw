// Object.freeze() is shallow
const obj = Object.freeze({});
obj.key = 42;
console.log(obj);   // {}