const restaurant = {
  soda: 2,
  burger: 1 
};

const restHandler = {
  get: function(target, property) {
    if (property in target && target[property] > 0) {
      target[property] -= 1;
      return `Enjoy your ${property}`;
    }
    return `Sorry, We are out of ${property}s!`;
  }
};
const restProxy = new Proxy(restaurant, restHandler);

console.log(restProxy.soda);
// Enjoy your soda
console.log(restProxy.soda);
// Enjoy your soda
console.log(restProxy.soda);
// Sorry, We are out of sodas!

console.log(restProxy.burger);
// Enjoy your burger
console.log(restProxy.burger);
// Sorry, We are out of burgers!

// intercept (or “trap”) native operations of the restaurant object 
// and execute the modified behavior on it

// traps all the “get property accessor” invocations
// the handler object contains a trap for the get property accessor 
// that receives the target and the property name when invoked. 