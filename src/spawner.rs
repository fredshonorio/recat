use std::sync::mpsc::{Sender, channel};
use std::thread;
use process::worker;

pub fn spawner(max_threads: i32, tx: &Sender<String>) -> () {
    let (close_tx, close_rx) = channel();
    let mut current = 0;

    loop {
        spawn_workers(max_threads - current, tx, &close_tx);
        current = max_threads;
        close_rx.recv().unwrap();
        current -= 1;
    }
}

fn spawn_workers(n_threads: i32, tx: &Sender<String>, close: &Sender<()>) -> () {
    for _ in 0..n_threads {
        let thread_tx = tx.clone();
        let close_tx = close.clone();
        thread::spawn(move || worker(&thread_tx, &close_tx));
    }
}
