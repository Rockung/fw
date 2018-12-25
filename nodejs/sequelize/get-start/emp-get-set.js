const sequelize = require('./db');
const Employee = sequelize.import('./employee');

Employee
  .sync({force: false})
  .then(() => {
    Employee.create({ name: 'John Doe', title: 'senior engineer' })
    .then(employee => {
      console.log(employee.get('name')); // John Doe (SENIOR ENGINEER)
      console.log(employee.get('title')); // SENIOR ENGINEER
    });  
  });
