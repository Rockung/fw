let arr = Array.of(10, 20, 30, 40);
console.log(arr); // [10, 20, 30, 40]

// You might be wondering how is this different from the traditional
// Array(...) constructor. You can also create an array with the array 
// constructor method using the Array() syntax. But, the difference 
// between Array.of() and the Array constructor is the way they handle
// a single number as an argument. The Array constructor has a very 
// weird behavior where if only one number is passed to it, instead of 
// making an array of one element with that number value, it constructs 
// an empty array with the number as its length. All the elements are 
// set to undefined.

const arr1 = Array.of(10);
console.log(arr1); // [10]
console.log(arr1.length);      // 1
const arr2 = Array(10);
console.log(arr2); // [,,,,,,,,,]
console.log(arr2.length);      // 10

// we need to understand why we would have to use the constructor in the 
// first place instead of a literal syntax like a = [1, 2, 3]. You would 
// use the constructor, for example, in case you have a callback that is 
// supposed to wrap arguments passed to it into an array or if you have 
// to subclass Array and want to create or initialize elements in an 
// instance of your subclass.