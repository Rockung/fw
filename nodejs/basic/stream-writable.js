const { Writable } = require('stream');

class MyWritable extends Writable {
	constructor(options) {
		super(options);
	}
	
	_write(chunk, encoding, callback) {
		// console.log(chunk.toString());
		if (chunk.toString().indexOf('a') < 0) {
			callback(new Error('chunk is invalid'));
		} else {
			callback();
		}
	}
}

const writer = new MyWritable();
writer.setDefaultEncoding('utf8');

writer.on('error', (err) => {
	console.log(err);
});
writer.on('finish', () => {
	console.log('All writes are now complete.');
});

writer.write('What are you doing?');
writer.end('This is the end. There is a A\n');
