var punt = require('punt');
var server = punt.bind('127.0.0.1:5000');

server.on('message', function(msg){
  console.log(msg);
});

