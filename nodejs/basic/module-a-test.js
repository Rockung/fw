const a = require('./module-a.js');

a.on('ready', () => {
  console.log('module a is ready');
});