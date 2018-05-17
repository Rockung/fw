// mpsc: multi-producer, single-consumer FIFO queue
//       unidirectional flow between Sender and Receiver

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // channel with two endpoints: Sender<T>, Receiver<T>
    // where messages with type T are transferred
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // multi-producer: the sender can be copied
        let thread_tx = tx.clone();

        thread::spawn(move || {
            // Non-blocking send returns a Result
            thread_tx.send(id).unwrap();
            // warning message when compiling
            // thread_tx.send(id);

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // blocking recv gets the messages from the channel
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}