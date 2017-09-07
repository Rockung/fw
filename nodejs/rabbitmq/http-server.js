var http = require('http'),
amqp = require('amqp');

var rabbit = amqp.createConnection();
rabbit.on('ready', function(){
	rabbit.exchange('my-first-exchange', {type: 'direct', autoDelete:
		false}, function(ex){
		startServer(ex);
	});
});

function startServer(ex) {
	var server = http.createServer(function(req, res){
		var msg = "this is a message with barcode!"
		console.log(msg);
		
		ex.publish('first-queue', {message: msg});
		res.writeHead(200, {'Content-Type': 'text/html'});
		res.end('<h1>Simple HTTP Server in Node.js!</h1>');
	});

	server.listen(3000);
}
