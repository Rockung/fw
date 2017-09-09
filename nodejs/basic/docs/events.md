Asynchronous event-driven architecture
---

        event
emitter ----> listener

Register a listener function
  EventEmitter.on()

Trigger an event
  EventEmitter.emit()

Asynchronous vs. Synchronous
---

The listeners are called synchronously in the order in which they were registered.

switch to asynchronous mode of operation

setImmediate()
process.nextTick()


