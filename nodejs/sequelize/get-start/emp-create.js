const Db = require('./db');
const Employee = Db.import('./employee');

Employee
.create({ name: 'John Doe', title: 'senior engineer' })
.then(employee => {
  console.log(employee.get('name')); // John Doe (SENIOR ENGINEER)
  console.log(employee.get('title')); // SENIOR ENGINEER
});  
