const sequelize = require('./db');
const User = sequelize.import('./user');

User.sync({force: true}).then(() => {
  return User.create({
    firstName: 'John',
    lastName: 'Hancock',
  });
});
