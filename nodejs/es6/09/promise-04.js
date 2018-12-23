const p1 = Promise.resolve(3);
const p2 = 42;
const p3 = new Promise((resolve, reject) => {
  setTimeout(resolve, 1000, "foo");
});

Promise.all([p1, p2, p3]).then(values => {
  console.log(values); // [3, 42, "foo"]
});

// Promise.all takes an array of promises (or any iterable) and returns a 
// promise that resolves when all of the promises in the iterable argument 
// have resolved, or rejects with the reason of the first passed promise 
// that rejects.