function *getFruits() {
  yield "apple";
  yield "orange";
  yield "banana";
  return "kiwi";
  yield "watermelon";
}

const fruitIterator = getFruits();
for (let fruit of fruitIterator) {
  console.log(fruit);
}

// traversing the sequence obtained from a generator does not include 
// the value that signals { done: true }.