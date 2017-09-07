var punt = require('punt');

var server = "172.27.35.1:5000";
var a = punt.connect(server);
var b = punt.connect(server);
var c = punt.connect(server);

var count = 0;

setInterval(function(){
  count = count + 1;
  a.send({ hello: 'world ' + count });
}, 150);

setInterval(function(){
  b.send('hello world');
}, 150);

setInterval(function(){
  c.send(new Buffer('hello'));
}, 150);
