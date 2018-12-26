module.exports = (sequelize, DataTypes) => {
  return sequelize.define('task', { name: DataTypes.STRING });
}
