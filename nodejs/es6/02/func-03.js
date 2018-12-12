var getPrice = (quantity, tax) => (quantity * 5) * (1 + tax);
console.log(getPrice(2, .095)); //  10.95