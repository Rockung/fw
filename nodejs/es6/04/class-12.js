class AeroPlane {
    constructor(capacity) {
        this.capacity = capacity;
    }
    showCapacity() {
        console.log(`Capacity of this plane: ${this.capacity}`);
    } 
}

// class FighterPlane {
//     constructor(capacity) {
//         this.capacity = capacity;
//     }
//     showCapacity() {
//         console.log(`Capacity of this plane: ${this.capacity}`);
//     }
//     fire() {
//         console.log("Loading weapons and firing");
//     }
// }

class FighterPlane extends AeroPlane {
    // no constructor
    fire() {
        console.log("Loading weapons and firing");
    }
}

// class FighterPlane extends AeroPlane {
//     constructor(...args) {
//         super(...args);
//     }
//     fire() {
//         console.log("Loading weapons and firing");
//     }
// }

const phantom = new FighterPlane(2);
phantom.showCapacity();
// Capacity of this plane: 2
phantom.fire();
// Loading weapons and firing
