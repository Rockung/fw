import { add as sum } from "./module-01.js";
console.log(typeof add);            // "undefined"
console.log(sum(1, 2));             // 3

// rename when importing

// babel-node --harmony module-08.js