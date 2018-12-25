module.exports = (sequelize, DataTypes) => {
  return sequelize.define('employee', {
    name: {
      type: DataTypes.STRING,
      allowNull: false,
      get() {
        const title = this.getDataValue('title');
        // 'this' allows you to access attributes of the instance
        return this.getDataValue('name') + ' (' + title + ')';
      },
    },
    title: {
      type: DataTypes.STRING,
      allowNull: false,
      set(val) {
        this.setDataValue('title', val.toUpperCase());
      }
    }
  });  
}

