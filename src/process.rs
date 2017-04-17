use std::sync::mpsc::Sender;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{BufReader, BufRead};

enum ProcResult {
    Exited,
    Err(String),
}

// stream all the output of a process and send a close signal when done
pub fn worker(cmd: String, tx: &Sender<String>, close: &Sender<()>) -> () {
    let res = stream_output(&tx, cmd);
    let exit = match res {
        ProcResult::Exited => "exited".to_string(),
        ProcResult::Err(msg) => format!("exited with error: {}", msg).to_string(),
    };
    tx.send(exit).unwrap();
    close.send(()).unwrap();
}

// send all lines of stdout and return a process result
fn stream_output(tx: &Sender<String>, cmd: String) -> ProcResult {

    let process = Command::new("/usr/bin/bash")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn();

    let child = match process {
        Ok(x) => x,
        Err(msg) => return ProcResult::Err(msg.description().to_string()),
    };

    let out = match child.stdout {
        Some(b) => b,
        None => return ProcResult::Exited,
    };

    for line in BufReader::new(out).lines() {
        match line {
            Ok(l) => tx.send(l).unwrap(),
            Err(_) => {}
        }
    }

    return ProcResult::Exited;
}
