use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let path_dipslay = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}",
                           path_dipslay,
                           err.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}",
                           path_dipslay,
                           err.description()),
        Ok(_) => print!("{} contains:\n{}", path_dipslay, s),
    };

    // `file` variable goes out of scope, so the "hello.txt" file gets closed
}
