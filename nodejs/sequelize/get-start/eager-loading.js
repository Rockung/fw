const Sequelize = require('sequelize');
const Op = Sequelize.Op;

const Db = require('./db');
const User = Db.import('./user');
const Task = Db.import('./task');
const Tool = Db.import('./tool');

Task.belongsTo(User);
User.hasMany(Task);
User.hasMany(Tool, { as: 'Instruments' });

// Task.findAll({ include: [ User ] }).then(tasks => {
//   console.log(JSON.stringify(tasks));
// })

// User.findAll({ include: [ Task ] }).then(users => {
//   console.log(JSON.stringify(users));
// })

// User.findAll({ include: [{ model: Tool, as: 'Instruments' }] }).then(users => {
//   console.log(JSON.stringify(users));
// })

// User.findAll({ include: ['Instruments'] }).then(users => {
//   console.log(JSON.stringify(users));
// })

// User.findAll({ include: [{ association: 'Instruments' }] }).then(users => {
//   console.log(JSON.stringify(users));
// })

// User.findAll({
//   include: [{
//     model: Tool,
//     as: 'Instruments',
//     where: { name: { [Op.like]: '%cil%' } }
//   }]
// }).then(users => {
//   console.log(JSON.stringify(users));
// })

User.findAll({
  where: {
    '$Instruments.name$': { [Op.iLike]: '%ooth%' }
  },
  include: [{
    model: Tool,
    as: 'Instruments'
  }]
}).then(users => {
  console.log(JSON.stringify(users));
})
