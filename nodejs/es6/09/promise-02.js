const pizza = new Promise((resolve) => {
  console.log("Getting your pizza in 5 seconds...");
  setTimeout(() => {
    resolve("Onion Pizza");
  }, 5000);
});

pizza.then(
  (item) => { console.log(`Order Received: ${item}`) },
  (error) => { console.log("Something went wrong with your pizza") }
);