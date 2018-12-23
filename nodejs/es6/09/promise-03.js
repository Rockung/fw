const bond = new Promise((resolve, reject) => {
  resolve("Bond");
});

bond.then((str) => `${str}, James ${str}`)
    .then((str) => `Hello, I’m ${str}!`)
    .then((str) => console.log(str));

// Chaining promises allows asynchronous operations to be chained together, 
// so that they are guaranteed to happen in the correct and expected order,
// resulting in code that looks almost synchronous. 