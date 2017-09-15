const { spawn } = require('child_process');
const subprocess = spawn('bad_command');

subprocess.on('error', (err) => {
	console.log('Failed to start subprocess.');
});
