var socket = require('socket.io-client')('http://localhost:3000');

socket.on('connect', function() {
	console.log('connect to the server');
	for (var i = 0; i < 1000; i++) {
		socket.emit('event', 'This is socket fore-end test for rabbitmq!');
	}
	socket.close();
});

socket.on('event', function(data) {
	
});

socket.on('disconnect', function() {
	
});
