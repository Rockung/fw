const cluster = require('cluster');

if (cluster.isMaster) {
	const worker = cluster.fork();

	worker.on('exit', (code, signal) => {
		if (signal) {
			console.log(`worker was killed by signal: ${signal}`);
		} else if (code !==0) {
			console.log(`worker exited with error code: ${code}`);
		} else {
			console.log('worker success!');
		}
	});	
} else {
	process.exit(1);
}
