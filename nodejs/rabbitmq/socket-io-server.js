var io = require('socket.io');
var amqp = require('amqp');
var rabbit = amqp.createConnection();

rabbit.on('ready', function(){
	rabbit.exchange('my-first-exchange', {type: 'direct', autoDelete: false}, function(ex) {
		startServer(ex);
	});
});

function startServer(ex) {
	console.log('starting a socket server ...');
	
	var server = io();
	server.on('connection', function(client) {
		console.log('a client log in');
		
		client.on('event', function(data) {
			console.log('data: ' + data);
			ex.publish('first-queue', {message: data});
		});
		
		client.on('disconnect', function(){
			console.log('client: closed');
		});	
	});
	
	server.listen(3000);
	console.log('listening on 3000 ...');
}
