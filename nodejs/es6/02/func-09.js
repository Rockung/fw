function Employee(name, department, salary) {
    this.name = name;
    this.department = department;
    this.salary = salary;
    console.log("Welcome " + this.name + "!");
}
let john = new Employee('John', 'Sales', 4000);
console.log(john.name) // Welcome John!