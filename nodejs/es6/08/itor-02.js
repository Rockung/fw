const arr = [41, 42, 43];
const itrObj = arr[Symbol.iterator]();
console.log(itrObj.next()); // { value: 41, done: false }
console.log(itrObj.next()); // { value: 42, done: false }
console.log(itrObj.next()); // { value: 43, done: false }
console.log(itrObj.next()); // { value: undefined, done: true }