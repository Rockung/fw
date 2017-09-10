const { Readable } = require('stream');

class Counter extends Readable {
  constructor(opt) {
	  super(opt);
		this._max = 1000000;
		this._index = 1;
	}
	
	_read() {
	  const i = this._index++;
		if (i > this._max)
			this.push(null);
		else {
		  const str = '' + i;
			const buf = Buffer.from(str, 'ascii');
			this.push(buf);
		}
	}
}

const counter = new Counter();
/*
counter.on('readable', () => {
  console.log(counter.read());
});
*/
let data = '';
counter.on('data', (chunk) => {
	data += chunk;
});
counter.on('end', () => {
	console.log(data);
	console.log('end');
});
