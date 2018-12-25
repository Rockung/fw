const sequelize = require('./db');
const User = sequelize.import('./user');
  
User.sync({force: false}).then(() => {
  return User.create({
    firstName: 'Kelvin',
    lastName: 'Peng',
  });
});
