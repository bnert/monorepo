use std::{thread, time};

fn main() {
    let mut i: u32 = 0;

    let out = thread::spawn(move || {
        loop {
            println!(">> out >> {}", i);
            i += 1;
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    out.join();
}
