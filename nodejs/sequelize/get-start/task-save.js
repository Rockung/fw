
const Db = require('./db');
const Task = Db.import('./task');

Task
  .build({ name: 'Cook eggs' })
  .save()
  .then(anotherTask => {
    console.log(anotherTask.get({plain: true}));
  })
  .catch(error => {
    console.log(error);
  })