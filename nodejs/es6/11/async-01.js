// Asynchronous functions
// * async - return a promise, which can be resolved to a value
// * await - suspend the execution until the promise is settled

function getData(site) {
  return fetch(site)
    .then(request => request.json())
}

async function getData(site) {
  let request = await fetch(url);
  let text = await request.text();
  return JSON.parse(text);
}

// use async in function expressions, method definitions, and arrow 
// functions like the following examples:

// Using function expression:
const getData = async function () {};
// Using method definition:
const item = { async getData() {} }
// Using arrow function:
const item = async () => {};

// the await keyword works with promises only, and casts the expression
// into a promise if it's not one.

// the await operator waits for the operand, that is, the promise to be 
// settled and if the promised is fulfilled, its result is the fulfillment
// value and if the promise is rejected, it throws the rejection value.
// Therefore, we can use a traditional try catch block with async/ await
// to handle the promises in a better way.
