use std::error::Error;
use std::io::prelude::*;
use std::process::{ Command, Stdio };

static PANGRAM: &'static str =
    "the quick brown fox jumed over the lazy dog\n";

fn main() {
    let process = Command::new("wc")
                          .stdin(Stdio::piped())
                          .stdout(Stdio::piped())
                          .spawn()
                          .unwrap_or_else(|e| {
        panic!("Couldn't spawn wc: {}", e.description());
    });

    // `stdin` has type `Option<ChildStdin>`,
    // and we know this instance must have one,
    // so we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(e) => panic!("Couldn't write to wc stdin: {}", e.description()),
        Ok(_)  => println!("Send program to wc."),
    }

    // Because `stdin` does not live after the above calls,
    // it is `drop`ed, and the pipe is closed.
    // this is very important, otherwise `wc` wouldn't start processing the input we just sent.

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(e) => panic!("Couldn't read wc stdout: {}", e.description()),
        Ok(_)  => print!("wc responded with:\n{}", s),
    }
}
