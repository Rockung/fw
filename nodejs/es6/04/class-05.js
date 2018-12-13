class AeroPlane {
    constructor(model, capacity) {
        this._model = model;
        this._capacity = capacity;
    }
    get model() {
        return this._model;
    }
    get capacity() {
        return this._capacity;
    }
    set model(model) {
        this._model = model;
    }
    set capacity(capacity) {
        this._capacity = capacity;
    } 
}
const jet = new AeroPlane("Jet", 100);
console.log(jet.capacity);
// 100
console.log(Object.getOwnPropertyDescriptor(AeroPlane.prototype, "model"));
/*
{
    get: [Function: get model],
    set: [Function: set model],
    enumerable: false,
    configurable: true
}
*/