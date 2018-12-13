const price = 4.20, quantity = 20;
const invoiceData = {
    price,
    quantity,
    calculateTotal() {
        return this.price * this.quantity;
    }
};
console.log(invoiceData.calculateTotal());
