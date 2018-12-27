## Preparing

* mkdir migration && cd migration
* cnpm install --save sequelize
* cnpm install --save sqlite3
* cnpm install --save-dev sequelize-cli

## Bootstrappping
* ./node_modules/.bin/sequelize init

or 

* ./node_modules/.bin/sequelize init:config
* ./node_modules/.bin/sequelize init:migrations
* ./node_modules/.bin/sequelize init:models
* ./node_modules/.bin/sequelize init:seeders

## Modifying in config/config.json
`
{
  "development": {
    "database": "dev",
    "dialect": "sqlite",
    "storage": "./db/dev.sqlite"
  },
  "test": {
    "database": "test",
    "dialect": "sqlite",
    "storage": "./db/test.sqlite"
  },
  "production": {
    "username": "root",
    "password": null,
    "database": "database_production",
    "host": "127.0.0.1",
    "dialect": "mysql"
  }
}
`

## Creating first model(or migration)

* ./node_modules/.bin/sequelize model:generate --name User --attributes firstName:string,lastName:string,email:string

## Running migrations

* ./node_modules/.bin/sequelize db:migrate

## Undoing migrations

* ./node_modules/.bin/sequelize db:migrate:undo

## The .sequelizerc File

* touch .sequelizerc

`
const path = require('path');

module.exports = {
  'config': path.resolve('config', 'database.json'),
  'models-path': path.resolve('db', 'models'),
  'seeders-path': path.resolve('db', 'seeders'),
  'migrations-path': path.resolve('db', 'migrations')
}
`

## Reference

* http://docs.sequelizejs.com/manual/tutorial/migrations.html
* https://github.com/sequelize/cli