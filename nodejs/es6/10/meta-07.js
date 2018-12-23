function getBill(amount) {
  return amount;
}
const billHandler = {
  apply: function(target, context, args) {
    console.log("Applying Discount of 35%");
    return args[0] - (args[0] * 0.35);
  }
}
const billProxy = new Proxy(getBill, billHandler);
console.log(billProxy(300));
// Applying Discount of 35%
// 195

// apply is used to trap a function invocation. It takes three arguments:
// • target:  the target function whose behavior is being modified;
// • context: the context passed as this to target on invocation;
// • args:    the arguments passed when applying the call.

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Meta_programming
