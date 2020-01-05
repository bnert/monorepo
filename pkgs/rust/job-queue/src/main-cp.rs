use std::time;
use std::thread;
use std::process::Command;
use std::sync::mpsc;

use std::io::{self, Write};

#[derive(Copy, Clone)]
struct Job {
    id: u16,
    script_filepath: &'static str
}

struct JobQueue {
    next_id: u16,
    jobs: Vec<Job>
}

impl JobQueue {
    fn new() -> JobQueue {
        JobQueue { next_id: 0, jobs: Vec::new() }
    }

    fn add_end(&mut self, script_filepath: &'static str) {
        self.jobs.push(Job { id: self.next_id, script_filepath: script_filepath });
        self.next_id += 1;
    }

    fn list(&self, stdout: &mut std::io::StdoutLock) {
        for job in &self.jobs {
            stdout.write(job.script_filepath.as_bytes());
            stdout.write(b"\n");
        }
    }

    fn jobs(&self) -> Vec<Job> {
        self.jobs
    }

    fn run(&self) {
        for job in &self.jobs {
            Command::new("sh")
                .arg(&job.script_filepath)
                .output()
                .expect("Unable to run process");
        }
    }
}


fn main() {
    let (tx, rx) = mpsc::channel();


    // Spawn a thread which will run all jobs each second
    let worker_thread = thread::spawn(move || {
        let recv = rx.recv().unwrap();
    });

    let input_thread = thread::spawn(move || {

        let stdout = io::stdout();
        let mut std_out = stdout.lock();
        let mut buffer: &'static String = &String::new();

        let mut jq = JobQueue::new();

        tx.send(jq.jobs());

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
                    jq.add_end(filepath);
                    std_out.write(b"Added ");
                    std_out.write(filepath.as_bytes());
                },
                "j" => {
                    jq.list(&mut std_out);
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
}
