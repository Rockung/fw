function myIterator(data) {
  let currentIndex = 0;
  return {
    next: () => {
      const done = (currentIndex >= data.length);
      const value = !done ? data[currentIndex] : undefined;
      currentIndex += 1;
      return {
        done,
        value
      }; 
    }
  };
}

const itrObj = myIterator([41, 42, 43]);
console.log(itrObj.next());  // { value: 41, done: false }
console.log(itrObj.next());  // { value: 42, done: false }
console.log(itrObj.next());  // { value: 43, done: false }
console.log(itrObj.next());  // { value: undefined, done: true }

// for all further calls
console.log(itrObj.next());    // { value: undefined, done: true }