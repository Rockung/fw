var defaultName = "John";
var getName = function(firstName = defaultName, lastName = "Doe") {
        console.log(firstName + " " + lastName);
}
getName(); // John Doe