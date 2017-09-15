process.on('message', (m, server) => {
	if (m === 'server') {
		server.on('connection', (socket) => {
			console.log('handled by child');
			socket.write('sent by child');
			socket.end('handled by child');
		});
	}
});
