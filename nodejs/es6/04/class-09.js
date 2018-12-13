const data = new WeakMap();
class AeroPlane {
    constructor(seats) {
        data.set(this, {
            capacity: seats
        }); 
    }
    get seats() {
        return data.get(this).capacity;
    }
    set seats(value) {
        data.get(this).capacity = value;
    } 
}
const jet = new AeroPlane(200);
console.log(jet.capacity); // undefined
console.log(jet.seats);    // 200