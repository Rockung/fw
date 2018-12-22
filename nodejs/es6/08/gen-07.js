function *getFruits() {
  try {
    yield "apple";
    yield "orange";
    yield "banana";
  }
  finally {
    console.log("You must always eat a big watermelon");
    yield "watermelon";
  }
}

const fruitIterator = getFruits();
console.log(fruitIterator.next());
// { value: 'apple', done: false }

console.log(fruitIterator.return("kiwi"));
// You must always eat a big watermelon
// { value: 'watermelon', done: false }

console.log(fruitIterator.next());
// { value: 'kiwi', done: true }

console.log(fruitIterator.next());
// { value: undefined, done: true }

// try-finally block ensuring that the code with the finally clause will 
// execute in case of completion, whether early or when the yield statements 
// are exhausted