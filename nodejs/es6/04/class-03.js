class AeroPlane {
    constructor(model, capacity) {
        this.model = model;
        this.capacity = capacity;
    }
    getData() {
        console.log(`You're flying a ${this.model} aeroplane`);
        console.log(`This plane can fly with ${this.capacity} passengers`);
    } 
}
const jet = new AeroPlane("Jet", 60);
jet.getData();
// You're flying a Jet aeroplane
// This plane can fly with 60 passengers
console.log(jet.hasOwnProperty("getData"));
console.log(jet.__proto__.hasOwnProperty("getData"));
// false
// true