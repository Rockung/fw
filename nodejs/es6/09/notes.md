## Asynchronous Programming

JavaScript, as we know it, is single threaded by design. In browsers, it uses event handling to manage a lot of tasks concurrently. The JavaScript Engine manages an event queue and when an event occurs, the registered handler function is called. If you have any experience with traditional JavaScript, then you are already well aware of the fact that understanding asynchronous programming is a must to properly be able to use JavaScript. The main pattern for using asynchrony is through the callback function.

## Callback Hell

All browser-based asynchronous functions are callback based. While the concept is simple and easy to understand in theory, it can lead to some really confusing and difficult-to-follow code,especially in cases when it is needed to make a callback after a callback (nested callbacks), which is more often termed “Callback Hell.”

This can get more and more complex in real-life applications where you have lots of callback functions, which can lead to a callback pyramid. These callback pyramids appear everywhere – in handling HTTP requests, database manipulation, animation, interprocess communication, and all manners of other places. Generally, codes using callback functions can become harder to follow, refactor, and test.

## Promises

ES6 brings to the standard JavaScript specification a new feature Promises (a feature available in some of the new and latest JS libraries and frameworks) that solves many of the significant problems in the callbacks – the only approach to async.

Promises give us a way to handle asynchronous processing in a more controlled pattern. They represent a value that can be handled at some point in the future, and offer an escape from the callback hell Promises provide a simpler alternative for executing, composing, and managing asynchronous operations in comparison to callback-based approaches. They also allow the logical flow of the code to be much easier to follow.

ES6 has native support for promises. A promise is an object that is waiting for an asynchronous operation to complete, and when that operation completes, the promise is either fulfilled or rejected. A promise object can be any of these three states:

* fulfilled – when the promise succeeds
* rejected – when the promise fails
* pending – when it’s neither fulfilled or rejected

A pending promise may transition into a fulfilled or rejected state, and the promise is considered to be settled when it’s either fulfilled or rejected. It is important to note a settled promise is immutable, which means that once a promise is settled, it cannot be resettled.

Promises let us write asynchronous code in a synchronous fashion, with flat indentation and a single exception channel.

## Creating a Promise

Promises are created using the new Promise() constructor that accepts an **executor** (a function) that takes two parameters:

* The first parameter (typically named **resolve**) is a function that is called with the future value when it's ready, that is, when the promise is fulfilled;
* And the second parameter (typically named **reject**) is a function that is called to reject the promise if it can't resolve the future value.

The **executor** initiates some asynchronous work and then, once that completes, calls either the resolve or reject function to resolve the promise or else reject it if an error occurred.
A simple Promise looks like this:

`
const p = new Promise((resolve, reject) => {
    if (/* some condition */) {
        resolve(/* some value */);  // fulfilled successfully
    } else {
        reject(/* some reason */);  // error, rejected
    }
});
`

The second method (reject) is optional,and you can very well create a promise with only the resolve method, as demonstrated in the code snippet below where the promises are fulfilled and rejected, respectively:

`
new Promise(resolve => resolve()) // promise is fulfilled
new Promise((resolve, reject) => reject()) // promise is rejected

const sayHello = Promise.resolve("hello!");
const sayHello = Promise.reject("hello!");
`

## Consuming a Promise

Once a promise is created, it can be passed around as a value, essentially representing a placeholder for a future value. This value can be consumed when the promise is fulfilled using .then() method. This method takes a function that will be passed to the resolved value of the Promise once it is fulfilled.

Every promise must have a .then() method that actually takes two possible parameters. The first parameter is the function to be called when the promise is fulfilled and the second parameter is a function to be called if the promise is rejected.

On the other hand, you can also handle a rejected promise in a more compact way using the catch() method. 

The catch() method is useful for error handling in promise composition. Similar to then() method, it also returns a promise, but only deals with the rejected cases. catch() method behaves as an abbreviation for then(null, onRejected). 

`
somePromise().then(onResolved, onRejected);
somePromise()
       .then(onResolved)
       .catch(onRejected);
`

But which one should you prefer and why? The answer to this question lies in the
very scenario where your onResolved() function throws an error. The promise returned from then() method will be rejected, but using the first way, the rejection cannot be caught—resulting in the error to get swallowed in your app. But using the second way, errors originating from both somePromise() and onResolved() can be handled at the catch() method. If we have more than one then() call, then the error is passed on until there is an error handler. Therefore, it is recommended to end all promise chains with a catch() method.
