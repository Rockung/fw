var punt = require('punt');
var amqp = require('amqp');
var rabbit = amqp.createConnection();


rabbit.on('ready', function(){
	rabbit.exchange('my-first-exchange', {type: 'direct', autoDelete: false}, function(ex) {
		startServer(ex);
	});
});

function startServer(ex) {
	var addr = '127.0.0.1:5000';
	var server = punt.bind(addr);

	server.on('message', function(msg){
		console.log(msg);
		ex.publish('first-queue', {message: msg});
	});

	console.log('UDP server bind on ' + addr);
}
