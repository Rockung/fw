const restaurant = {
  soda: 10,
  beer: 5 
};

const { proxy, revoke } = Proxy.revocable(restaurant, {});

console.log(proxy.soda);
console.log(proxy.beer);
revoke();
console.log(proxy.soda);
