class AeroPlane {
    constructor(capacity) {
        this.capacity = capacity;
    }
    showCapacity() {
        console.log(`Capacity of this plane: ${this.capacity}`);
    }
    fly() {
        console.log("Engines on, and the plane will take off soon");
    }
}

class FighterPlane extends AeroPlane {
    fly() {
        super.fly();
        console.log("Engines on, and the plane is gone");
    }
    fire() {
        console.log("Loading weapons and firing");
    }
}

const phantom = new FighterPlane(2);
phantom.fly();
// Engines on, and the plane is gone
phantom.showCapacity();
// 2
