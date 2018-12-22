const message = "Ian is an awesome student";
const itrObj = message[Symbol.iterator]();
console.log(itrObj.next());      // { value: "I", done: false }
console.log(itrObj.next());      // { value: "a", done: false }