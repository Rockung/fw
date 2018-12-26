const Db = require('./db');
const Employee = Db.import('./employee');
  
Employee.findAll().then(employees => {
  for (employee of employees) {
    console.log(employee.get('name'));
  }
});