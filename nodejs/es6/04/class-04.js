// ES5 style
function AeroPlane(model, capacity) {
    this.model = model;
    this.capacity = capacity;
}
AeroPlane.prototype.getData = function() {
    console.log("You're flying a " + this.model + " aeroplane");
    console.log("This plane can fly with " + this.capacity + " passengers");
}
var jet = new AeroPlane("Jet", 60);
jet.getData();
// You're flying a Jet aeroplane
// This plane can fly with 60 passengers