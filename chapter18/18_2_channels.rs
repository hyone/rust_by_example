use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        thread::spawn(move || {
            // the thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation,
            // the thread will continue immediately after sending its message
            println!("thread {} finished", id);
        });

        // // Using `thread_tx` after moving closure causes moved value error.
        // println!("{:?}", thread_tx);
        // //=> error: use of moved value: `thread_t
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` methods picks a message from the channel.
        // `recv` will block the current thread if there are no messages available.
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}
