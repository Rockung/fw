const restaurant = {
  soda: 5
};

const restHandler = {
  set: function(target, property, value) {
    target[property] = value;
    console.log(`${property} has been added to inventory`);
  }
}

const restProxy = new Proxy(restaurant, restHandler);
restProxy.beer = 10;

// beer has been added to inventory
console.log(restaurant.beer)