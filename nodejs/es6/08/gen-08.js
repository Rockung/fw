function *getFruits() {
  yield "apple";
  yield "orange";
  yield "spinach";
  yield "watermelon"
}

const fruitIterator = getFruits(); 
for (let fruit of fruitIterator) {
  try {
    console.log(fruit);
    if (fruit === "spinach") {
      fruitIterator.throw("Vegetable Found");
    }
  }
  catch (err) {
    console.log(`Exception: ${err}`);
  }
}
console.log("the end");

// throw(...) also stops the execution of the generator but it is never called automatically.
// utilizing a try-catch block to enable error handling in generators