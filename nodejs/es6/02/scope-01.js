var price = 10; // Global Declaration
function showPrice() {
        var price = 12; // Local Declaration using var
        console.log(price); // 12
}
showPrice();
console.log(price); // 10