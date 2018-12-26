const Db = require('./db');
const Employee = Db.import('./employee');

Employee.findOne().then(employee => {
  console.log(employee.get('name'));
});

async function findOne() {
  employee = await Employee.findOne();
  console.log(employee.get('name'));
}

findOne();