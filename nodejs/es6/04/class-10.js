class AeroPlane {
    constructor(capacity) {
        this.capacity = capacity;
    }
    static radio(message) {
        console.log(`Message from broadcast: ${message}`)
    } 
}
AeroPlane.radio("Sky is clear");
// Message from broadcast: Sky is clear