mod process;
mod spawner;

use std::thread;
use std::sync::mpsc;
use spawner::spawner;

static NTHREADS: i32 = 3;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || spawner(NTHREADS, &tx));

    loop {
        let line = rx.recv();
        println!("{:?}", line);
    }
}
