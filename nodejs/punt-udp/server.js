var punt = require('punt');
var addr = '172.27.35.1:5000';
var server = punt.bind(addr);

server.on('message', function(msg){
  console.log(msg);
});

