const Sequelize = require('sequelize');
const Op = Sequelize.Op;

const Db = require('./db');
const User = Db.import('./user');
const Task = Db.import('./task');
const Tool = Db.import('./tool');

Task.belongsTo(User);
User.hasMany(Task);
User.hasMany(Tool, { as: 'Instruments' });

async function findAndUpdate() {
  const user = await User.findOne({
    where: { name: { [Op.like]: '%Peng%' } },
  });
  const task = await Task.findOne({
    include: [ User ],
    where: { name: { [Op.like]: '%egg%' } }
  });
  return await task.setUser(user);
}

findAndUpdate().then(task => {
  console.log(task);
})