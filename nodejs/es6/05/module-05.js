import multiply, { message } from "./module-01.js";

console.log(multiply (21, 2));    // 42
console.log(message);             // "42 is the answer to the everything."

// use a comma to separate the default local name and the non-default identifiers inside curly braces
// make sure to always have the default before the non- default values.

// babel-node --harmony module-05.js