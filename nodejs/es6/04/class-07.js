const methodName = "getColor";
const propName = "color";
class AeroPlane {
    constructor(color) {
        this._color = color
    }
    [methodName]() {
        return this._color;
    }
    get [propName]() {
        return this[`_${propName}`];
    }
    set [propName](value) {
        return this[`_${propName}`] = value;
    }
}
const whiteJet = new AeroPlane("white");
console.log(whiteJet.color);
// white