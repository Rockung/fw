const { fork } = require('child_process');
const normal = fork('child-process-child-socket.js');
const special = fork('child-process-child-socket.js');

const server = require('net').createServer({ pauseOnConnect: true });
server.on('connection', (socket) => {
	if (socket.remoteAddress === '127.0.0.1') {
		special.send('socket', socket);
		return;
	}
	
	normal.send('socket', socket);
});
server.listen(1337);