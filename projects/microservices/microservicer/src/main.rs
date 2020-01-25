use std::process;

mod conn_mgr;

fn main() {
    ctrlc::set_handler(move || {
        println!("Shutting down processes...");
        process::exit(0);
    })
    .expect("Unable to set ctrl-c handler");

}
