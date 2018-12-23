let cake = {};

console.log(cake);  // {}

Object.defineProperty(cake, "ingredients", {
  value: ["sugar", "all things nice"],
  enumerable: true,
  configurable: true,
  writable: true
});

console.log(cake); // { ingredients: [ 'sugar', 'all things nice' ] }
