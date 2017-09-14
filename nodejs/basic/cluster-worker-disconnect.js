const cluster = require('cluster');

if (cluster.isMaster) {
	const worker = cluster.fork();
	let timeout;
	
	worker.on('listening', (address) => {
		worker.send('shutdown');
		worker.disconnect();
		timeout = setTimeout(() => {
			worker.kill();
		}, 2000);
	});
	
	worker.on('disconnect', () => {
		console.log('on diconnect in worker');
		clearTimeout(timeout);
	});
} else if (cluster.isWorker) {
	const net = require('net');
	const server = net.createServer((socket) => {
		
	});
	server.listen(8000);
	
	process.on('message', (msg) => {
		if (msg === 'shutdown') {
			console.log('Gracefully shutdown!')
			process.exit();
		}
	});
}
