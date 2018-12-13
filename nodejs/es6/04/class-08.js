class AeroPlane {
    constructor(capacity) {
        this.checkCapacity = function(value) {
            if (capacity >= value) {
                return true;
            }
            return false;
        } 
    }
}
const jet = new AeroPlane(200);
console.log(jet.checkCapacity(100));
console.log(jet.capacity);
// true
// undefined