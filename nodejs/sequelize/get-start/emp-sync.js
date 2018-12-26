const Db = require('./db');
const Employee = Db.import('./employee');

Employee
  .sync({force: true})
  .then(() => {
  });
