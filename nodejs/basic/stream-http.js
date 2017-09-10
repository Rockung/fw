const http = require('http');

const server = http.createServer((req, res) => {
	// req is an http.IncomingMessage, which is a Readable stream
	// res is an http.ServerResponse, which is a Writable stream
	
	let body = '';
	
	// If an encoding is not set, Buffer objects will be received.
	req.setEncoding('utf8');

  // Readable streams emit 'data' events once a listener is added	
	req.on('data', (chunk) => {
		body += chunk;
	});
	
	// the end event indicates that the entire body has been received
	req.on('end', () => {
		try {
			const data = JSON.parse(body);
			res.write(typeof data);
			res.end('\r\n');
		} catch (err) {
			res.statusCode = 400;
			return res.end(`error: ${err.message}`);
		}
	});
});

server.listen(1337);

// $ curl localhost:1337 -d "{}"
// $ curl localhost:1337 -d "\"foo\""
// $ curl localhost:1337 -d "not json"
