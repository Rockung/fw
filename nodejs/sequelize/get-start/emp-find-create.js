const Db = require('./db');
const Employee = Db.import('./employee');

Employee
  .findOrCreate({where: {name: 'sdepold'}, defaults: {title: 'Technical Lead JavaScript'}})
  .spread((employee, created) => {
    console.log(employee.get({ plain: true}));
    console.log(created);
  });