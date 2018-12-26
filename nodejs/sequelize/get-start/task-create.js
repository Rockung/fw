const Db = require('./db');
const Task = Db.import('./task');

Task
  .create({ name: 'Do homework' })
  .then(task => {
    console.log(task.get('name'));
  });
