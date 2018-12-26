const Db = require('./db');
const User = Db.import('./user');

User
  .create({ name: 'John Doe' })
  .then(user => {
    console.log(user.get('name'));
  });
