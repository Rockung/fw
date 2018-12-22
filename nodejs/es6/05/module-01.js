export var text = "ES6 is awesome";
export let name = "Ian Murawski";
export const number = 7;
export let message = "42 is the answer to the everything.";

export function add(a, b) {
    return a + b;
}

export class Rectangle {
    constructor(length, width) {
        this.length = length;
        this.width = width;
    }
}

function multiply(a, b) {
    return a * b;
}
export { multiply };

export default function(a, b) {
    return a * b;
}
