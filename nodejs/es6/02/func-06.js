var fn = ((number) => {
    return {
        getNumber: function() {
            return number;
} };
})(42);
console.log(fn.getNumber());
// 42