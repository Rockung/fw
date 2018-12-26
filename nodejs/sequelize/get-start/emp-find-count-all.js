const Sequelize = require('sequelize');
const Op = Sequelize.Op;

const Db = require('./db');
const Employee = Db.import('./employee');

Employee
  .findAndCountAll({
     where: {
        title: {
          [Op.like]: 'SEN%'
        }
     },
     offset: 0,
     limit: 2
  })
  .then(result => {
    console.log(result.count);
    console.log(result.rows);
  });