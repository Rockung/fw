const field = 'dynamicRandom';
const price = 5.99;
const quantity = 2;
const invoiceData = {
    [field]: price,
    quantity,
    calculateTotal() {
        return this.price * this.quantity;
    }
};
console.log(invoiceData);
// { dynamicRandom: 5.99,
//   quantity: 2,
//   caculateTotal: [Function: calculateTotal] }
