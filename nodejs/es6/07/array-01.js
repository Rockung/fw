const customers = {
  '0': 'matt',
  '1': 'ian',
  '2': 'mikhail',
  '3': 'akia',
  '4': 'vincent',
  length: 5
};

Array.from(customers).forEach(customer => {
  console.log(customer);
});

Array.from(customers, customer => {
  console.log(customer);
});

// Array.from(arrayLike[, mapFn[, thisArg]])

// Array.from() tries to check if its first argument is an iterable, 
// and if it is it uses the iterator to produce values to copy into 
// the returned array. But if you pass an array-like object, it behaves
// the same as slice() or apply() does, which is simply loop over the
// values accessing numerically names properties from 0 to the length 
// of the object.