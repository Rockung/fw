var socket = require('engine.io-client')('ws://localhost:8000');
socket.on('open', function(){
  socket.on('message', function(data){
      console.log(data);
  });
  socket.on('close', function(){
      console.log("bye");
  });
});