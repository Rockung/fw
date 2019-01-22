const low = require('lowdb')
const FileSync = require('lowdb/adapters/FileSync')
 
const adapter = new FileSync('db.json')
const db = low(adapter)
 
// Set some defaults
// db.defaults({ posts: [], user: {} })
//   .write()

console.log(db.get('posts')
  .find({ id: 1 })
  .value())
