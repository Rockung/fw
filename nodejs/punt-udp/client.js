var punt = require('punt');

var a = punt.connect('127.0.0.1:5000');
var b = punt.connect('127.0.0.1:5000');
var c = punt.connect('127.0.0.1:5000');

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
