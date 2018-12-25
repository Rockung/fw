const sequelize = require('./db');
const User = sequelize.import('./user');

User.findOne().then(user => {
  console.log(user.get('firstName'));
});

async function findOne() {
  user = await User.findOne();
  console.log(user.get('firstName'));
}

findOne();