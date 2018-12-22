// import everything
import * as example from "./module-01.js";
console.log(example.add(1,7));          // 8
console.log(example.multiply(2, 3));    // 6

// all exported bindings are loaded into an object called 'example'
// all exports become accessible as properties of the declared object
// 'import' statement will only be executed once event importing repeatly
// the module is stored in memory and reused for subsequent imports

// babel-node --harmony module-03.js

