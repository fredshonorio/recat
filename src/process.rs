use std::sync::mpsc::Sender;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{BufReader, BufRead};

enum ProcResult {
    Exited,
    Err(String),
}

pub fn worker(tx: &Sender<String>, close: &Sender<()>) -> () {
    let res = stream_output(&tx);
    let exit = match res {
        ProcResult::Exited => String::from("exited"),
        ProcResult::Err(msg) => String::from(format!("exited with error: {}", msg)),
    };

    tx.send(exit).unwrap();
    close.send(()).unwrap();
}

fn stream_output(tx: &Sender<String>) -> ProcResult {

    let process = Command::new("/usr/bin/ls").stdin(Stdio::null()).stdout(Stdio::piped()).spawn();

    let child = match process {
        Ok(x) => x,
        Err(msg) => return ProcResult::Err(String::from(msg.description())),
    };

    let buffer = match child.stdout {
        Some(b) => b,
        None => return ProcResult::Exited,
    };

    let reader = BufReader::new(buffer);

    for line in reader.lines() {
        match line {
            Ok(l) => tx.send(l).unwrap(),
            Err(_) => {}
        }
    }

    return ProcResult::Exited;
}
