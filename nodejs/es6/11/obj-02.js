const myObj = {
  [Symbol('mySymbol')]: 42,
  get random() { return 'test' },
};
console.log(Object.getOwnPropertyDescriptors(myObj));