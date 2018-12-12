function Employee(firstName, department, salary) {
    this.firstName = firstName;
    this.department = department;
    this.salary = salary;
    this.getInfo = function() {
        // outer function context = Employee object
        return function () {
            // inner function context = Global object
            console.log(this.firstName + " from " +
            this.department + " earns " + this.salary);
        };
    }
}
let jim = new Employee('Jim', 'Finance', 5200);
let printInfo = jim.getInfo();
printInfo();