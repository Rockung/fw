const restaurant = {
  soda: 5,
  beer: 10 
};

const restHandler = {
  has: function(target, property) {
    if (property === "beer") {
      return false;
    }
    return property in target;
  }
}

const restProxy = new Proxy(restaurant, restHandler);
console.log("beer" in restProxy);    // false
console.log("soda" in restProxy);    // true