module.exports = (sequelize, DataTypes) => {
  return sequelize.define('tool', { name: DataTypes.STRING });
}
