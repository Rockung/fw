const Db = require('./db');
const Tool = Db.import('./tool');

// Tool
//   .create({ name: 'Pencil' })
//   .then(tool => {
//     console.log(tool.get('name'));
//   });

Tool
  .create({ name: 'Notebook' })
  .then(tool => {
    console.log(tool.get('name'));
  });

