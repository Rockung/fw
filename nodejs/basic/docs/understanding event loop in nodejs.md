understanding event loop in nodejs

each phase has a FIFO queue of callbacks to execute
phases of the event loop

* timers: executes callbacks scheduled by setTimeout()/setInterval()
* I/O callbacks: executes almost all callbacks
* idle, prepare: used only internally
* poll: retrieve new I/O events
* check: setImmediate() callbacks are invoked
* close callbacks: e.g. socket.on('close', ...)

setImmediate() vs setTimeout()

* the executing order will vary depending on the context
* within an I/O cycle, the immediate callback always first

understanding process.nextTick()

* the nextTickQueue processed after current operation completes, regardless of the current phase of the event loop
* it's allowed to make recursive calls to process.nextTick() without reaching a RangeError: Maximum call stack size exceeded from v8
* it's allowed to initialize variables, functions, etc. prior to the callback being called

process.nextTick() vs setImmediate()

* process.nextTick() fires immediately on the same phase
* setImmediate() fires on the following iteration or 'tick' of the event loop

why use process.nextTick()?

* allow users to handle errors, cleanup any then unneeded resources, or perhaps try the request again before the event loop continues
* at times it's necessary to allow a callback to run after the call stack has unwound but before the event loop continues

```
const EventEmitter = require('events');
const util = require('util');

function MyEmitter() {
  EventEmitter.call(this);

  // use nextTick to emit the event once a handler is assigned
  process.nextTick(() => {
    this.emit('event');
  });
}
util.inherits(MyEmitter, EventEmitter);

const myEmitter = new MyEmitter();
myEmitter.on('event', () => {
  console.log('an event occurred!');
});
```


