const Db = require('./db');
const Task = Db.import('./task');

const task = Task.build({name: 'Sweep desk'});
// console.log(task.id);
// console.log(task.name);
console.log(task);