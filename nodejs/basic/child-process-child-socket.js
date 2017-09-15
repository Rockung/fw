process.on('message', (m, socket) => {
	if (m === 'socket') {
		if (socket) {
			socket.write('handled by child\n');
			socket.end(`Request handled with ${process.argv[2]} priority\n`);
		}
	}
});
