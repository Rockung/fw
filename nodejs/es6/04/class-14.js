class AeroPlane {
    constructor(capacity, color) {
        this.capacity = capacity;
        this.color = color;
    } 
}
class FighterPlane extends AeroPlane {
    constructor(color) {
        // This fighterplane is 2-seater
        super(2, color);
    }
}
const phantom = new FighterPlane("grey");
console.log(phantom.capacity);       // 2
