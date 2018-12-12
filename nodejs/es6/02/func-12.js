function Employee() {
    this.firstName = 'Mike',
    this.department = 'HR',
    this.salary = 4500,
    this.getContext = () => {
        console.log(this);
    } 
}

let mark = new Employee();
mark.getContext(); // [Employee object]
let context = mark.getContext;
context(); // [Employee object] (regardless of a function invocation)
