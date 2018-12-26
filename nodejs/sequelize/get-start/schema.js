const Db = require('./db');

const User = Db.import('./user');
const Task = Db.import('./task');
const Tool = Db.import('./tool');

Task.belongsTo(User);
User.hasMany(Task);
User.hasMany(Tool, { as: 'Instruments' });

Db.sync().then(() => {
  // this is where we continue ...
});