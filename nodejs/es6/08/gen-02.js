// Note that yield is not just a pause point, it also sends out a 
// value when pausing the generator.

// Each time yield is called, the yielded value becomes the next value
// in the sequence. A yield statement without a value provided just 
// implies that the value is undefined.

function *infiniteNumbers() {
  var n = 1;
  while (true) {
    yield n++;
  } 
}

var numbers = infiniteNumbers(); // returns an iterable object
console.log(numbers.next());     // { value: 1, done: false }
console.log(numbers.next());     // { value: 2, done: false }
console.log(numbers.next());     // { value: 3, done: false }
