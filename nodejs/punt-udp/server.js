var punt = require('punt');
var addr = '127.0.0.1:5000';
var server = punt.bind(addr);

server.on('message', function(msg){
  console.log(msg);
});

