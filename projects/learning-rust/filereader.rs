//use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let s = [120, 0].into();
    println!("{}", s);
    // let four_k_bytes = 1024 * 4;
    // let f = File::open("aidans_theme-inst.wav").unwrap();
    // let mut buf: Vec<u8> = Vec::<u8>::with_capacity(four_k_bytes);

    // let mut i = 0;

    // for byte in f.bytes() {
    //     buf.push(byte.unwrap());
    //     i += 1;
    //     if i % four_k_bytes == 0 {
    //         println!("{}K bytes read", i / four_k_bytes);
    //     }
    // }

    // println!("Length: {}", buf.len());
    // let mut reader = BufReader::with_capacity(4, f);

    // loop {
    //     match reader.fill_buf() {
    //         Ok(_) => println!("{:?}", reader.buffer()),
    //         _ => break
    //     }
    // }
}
