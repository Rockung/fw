
class Car {
    constructor(brand) {
        this.brand = brand;
    }
    start() {
        console.log(`Your ${this.brand} is ready to go!`);
    }
}
const myTesla = new Car("Tesla");

myTesla.start();
// Your Tesla is ready to go!