// Iterator as an Iterable

const infiniteSequenceGenerator = {
  currentNumber: 0,
  // making the "infiniteSequenceGenerator" iterator an iterable
  [Symbol.iterator]() { return this; },
  next() {
    return {
      value: this.currentNumber++,
      done: false
    }
  }
};

const iter = infiniteSequenceGenerator[Symbol.iterator]();
console.log(iter === infiniteSequenceGenerator);
// true
console.log(iter.next().value);
console.log(iter.next().value);
console.log(iter.next().value);
console.log(iter.next().value);

for (let item of iter) {
  if (item > 20) break;
  console.log( item );
}

// Iterator Protocol
// define a next() method that takes no arguments and returns an 
// object with two properties: done and value.

// Iterable Protocol
// implement the @@iterator method using the [Symbol.iterator] property
// it can directly be used with the for...of loop