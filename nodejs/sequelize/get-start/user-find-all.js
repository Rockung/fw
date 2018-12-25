const sequelize = require('./db');
const User = sequelize.import('./user');
  
User.findAll().then(users => {
  console.log(users)
})