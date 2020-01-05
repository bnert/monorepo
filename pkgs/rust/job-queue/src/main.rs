use std::time;
use std::thread;
use std::process::Command;
use std::sync::mpsc;

use std::io::{self, Write};


fn main() {
    let (tx, rx) = mpsc::channel();

    let stdout = io::stdout();

    // Spawn a thread which will run all jobs each second
    let worker_thread = thread::spawn(move || {
        let _recv = rx.recv().unwrap();
    });

    let input_thread = thread::spawn(move || {
        let mut std_out = stdout.lock();
        let mut buffer = String::new();

        tx.send("Init");

        loop {
            std_out.write(b"jq> ");
            std_out.flush();

            io::stdin().read_line(&mut buffer).unwrap();

            // Need to strip the newlines
            let cmdChunks: Vec<&str> = buffer.trim_right_matches(|c| {
                c == '\r' || c == '\n'
            })
            .split(' ')
            .collect();

            let cmd: &str = cmdChunks[0];

            match cmd {
                "a" => {
                    let filepath: &str = cmdChunks[1];
                    tx.send(filepath);
                    std_out.write(b"Added ");
                    std_out.write(filepath.as_bytes());
                },
                "j" => {
                    tx.send("list");
                },
                ".q" => break,
                _ => {
                    std_out.write_all(cmd.as_bytes());
                    std_out.write_all(b"NOT RECOGNIZED\n");
                }
            }

            std_out.write(b"\n");
            std_out.flush();
            buffer.clear();
        }
    });

    worker_thread.join().unwrap();
    input_thread.join().unwrap();
}
