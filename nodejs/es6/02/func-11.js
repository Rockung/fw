function Employee(firstName, department, salary) {
    this.firstName = firstName;
    this.department = department;
    this.salary = salary;
    this.getInfo = function() {
        // outer function context = Employee object
        return () => {
            // inner function context = surrounding context = Employee object
            console.log(this.firstName + " from " + this.department + 
            " earns " + this.salary);
        };
} }
let jim = new Employee('Jim', 'Finance', 5200);
let printInfo = jim.getInfo();
printInfo();  // Jim from Finance earns 5200