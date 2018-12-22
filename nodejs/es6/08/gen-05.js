// Completing early

function *getFruits() {
  yield "apple";
  yield "orange";
  yield "banana";
}
const fruitIterator = getFruits();
console.log(fruitIterator.next()); 
console.log(fruitIterator.return("kiwi")); // {value: 'kiwi', done: true} 
console.log(fruitIterator.next());         // {value: undefined, done: true}
