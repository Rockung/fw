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
  },

  ownKeys: function(target) {
    return ["soda"];
  }
}

const restProxy = new Proxy(restaurant, restHandler);
console.log("beer" in restProxy);          // false
console.log(Object.keys(restProxy));       // ["soda"]

// ownKeys is used to trap the access of the owned properties and owned 
// symbol properties via Object.keys(), Object.getOwnPropertyNames() or 
// Object. getOwnSymbolProperties().