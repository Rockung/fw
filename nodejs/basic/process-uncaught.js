var fs = require('fs');

process.on('uncaughtException', (err) => {
  fs.writeSync(1, `Caught exception: ${err}\n`);
	// console.log(`Caught exception: ${err}\n`);
});

setTimeout(() => {
	console.log('This will still run.');
}, 500);

nonexistentFunc();
console.log('This will not run.');
