Streams can be readable, writable, or both.
All streams are instances of EventEmitter.

Types of Streams

* Readable
* Writable
* Duplex
* Transform: modify or transform the data

Object mode & buffer

            Writable            Readable
=========================================
Events      close               close
            drain               data
						finish              end
						pipe                readable
						unpipe              error
						error
