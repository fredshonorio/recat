use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;
use process::worker;

pub fn spawner(max_threads: i16, tx: &Sender<String>, timeout: Duration, cmds: Vec<String>) {
    let (close_tx, close_rx) = channel();
    let mut current: i16 = 0;
    let mut commands = cmds.iter().cycle();

    loop {
        // spawn all workers
        while current < max_threads {
            let next: &String = commands.next().unwrap();
            spawn_worker(next, tx, &close_tx);
            current += 1;
        }

        // wait for a worker to stop
        close_rx.recv().unwrap();
        current -= 1;
        thread::sleep(timeout);
    }
}

fn spawn_worker(cmd: &String, tx: &Sender<String>, close: &Sender<()>) {
    let thread_tx = tx.clone();
    let thread_close = close.clone();
    let thread_cmd = cmd.clone();
    thread::spawn(move || worker(thread_cmd, &thread_tx, &thread_close));
}
