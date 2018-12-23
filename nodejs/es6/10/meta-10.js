const restaurant = {
  soda: 2,
  beer: 5 
};

const restHandler = {
  get: function(target, property) {
    if (property === "beer") {
      return undefined;
    }
    return Reflect.get(target, property);
  }
};

const restProxy = new Proxy(restaurant, restHandler);
console.log(restProxy.beer);
// undefined
console.log(restProxy.soda);
