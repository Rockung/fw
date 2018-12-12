var details = {
        number: 42,
        operation: function () {
                return () => console.log(this.number);
} };
var details2 = {
        number: 84
};
details.operation().bind(details2)(); // 42

// So with an arrow function, calls to bind, call, or apply 
// will not be able to change to value of this.