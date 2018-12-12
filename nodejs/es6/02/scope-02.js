// IIFE: immediately invoked function expressions
var price = 10; // Global Declaration
(function () {
        var price = 12; // Local Declaration using var
        console.log(price); // 12
})();
console.log(price); // 10