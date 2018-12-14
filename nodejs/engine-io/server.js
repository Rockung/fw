var engine = require('engine.io');
var server = engine.listen(8000);

server.on('connection', function(socket) {
  socket.send('utf 8 string');
  socket.send(Buffer.from([0, 1, 2, 3, 4, 5]));
});