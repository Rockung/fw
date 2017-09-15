const subprocess = require('child_process').fork('child-process-child.js');

const server = require('net').createServer();

server.on('connection', (socket) => {
	console.log('handled by parent');
	socket.end('handled by parent');
});

server.listen(1337, () => {
	subprocess.send('server', server);
});
