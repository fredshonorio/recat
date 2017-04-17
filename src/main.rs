extern crate argparse;

mod process;
mod spawner;

use argparse::{ArgumentParser, Store, List, StoreOption};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use spawner::spawner;

fn parse_opts(timeout: &mut u64, cmds: &mut Vec<String>, n_concurrent: &mut Option<i16>) {
    let mut parser = ArgumentParser::new();
    parser
        .refer(timeout)
        .add_option(&["-t", "--timeout"],
                    Store,
                    "Set a timeout for a stopped process, in milliseconds. Default is 5000");
    parser
        .refer(cmds)
        .add_option(&["-c"],
                    List,
                    "The command to run, can be used multiple times.");
    parser
        .refer(n_concurrent)
        .add_option(&["-n"],
                    StoreOption,
                    "The maximum number of concurrent commands. By default it is the number of defined commands (-c switches).");
    parser.parse_args_or_exit();
}

fn main() {
    let mut timeout_opt: u64 = 5000;
    let mut cmds = Vec::new();
    let mut n_concurrent = None;
    parse_opts(&mut timeout_opt, &mut cmds, &mut n_concurrent);

    let concurrent_procs = n_concurrent.unwrap_or(cmds.len() as i16);

    if concurrent_procs <= 0 || cmds.len() == 0 {
        println!("TODO: <USAGE>");
        return;
    }

    let (tx, rx) = mpsc::channel();
    let timeout = Duration::from_millis(timeout_opt);

    thread::spawn(move || spawner(concurrent_procs, &tx, timeout, cmds));

    loop {
        let line = rx.recv();
        println!("{}", line.unwrap());
    }
}
