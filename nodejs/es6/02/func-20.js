var getFirstName = () => "John";
var getName = function(firstName = getFirstName(), lastName = "Doe") {
        console.log(firstName + " " + lastName);
}
getName(); // John Doe