// Generators
// functions that can be paused resume cycle midway through executions
// allows us to create a special type of iterator, whose execution can 
//   be suspended and retained while keeping the context
// it contains one or more yield expressions
// it uses the function * syntax

function *gen() {
  yield "Hello";
  yield "from";
  yield "generator";
}

// it will not be executed; instead it returns an iterator that will be 
// used to execute the code inside it.
let obj = gen();

console.log(obj.next());
console.log(obj.next());
console.log(obj.next());
console.log(obj.next());
