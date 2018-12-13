class Car {
    constructor(brand) {
        this.brand = brand;
    }
}
const myTesla = new Car("Tesla");
console.log(myTesla.hasOwnProperty("brand"));  // true
console.log(typeof Car);                       // function