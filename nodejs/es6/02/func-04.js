var getPrice = (quantity, tax) => {
  let price = (quantity * 5)
  price *= (1 + tax);
  return price;
}
console.log(getPrice(2, .095)); //  10.95
