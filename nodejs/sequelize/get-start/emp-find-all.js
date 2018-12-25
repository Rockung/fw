const sequelize = require('./db');
const Employee = sequelize.import('./employee');
  
Employee.findAll().then(employees => {
  for (employee of employees) {
    console.log(employee.get('name'));
  }
});