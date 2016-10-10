// Improve Sample Code at '16.5.3 Enter try!'
// ( http://rustbyexample.com/error/option_with_result/enter_try.html )
// to treat various Error types by using `Box<Error>`

use std::io::prelude::*;
use std::error::Error;
use std::fs::{ File, OpenOptions };

type Result<T> = std::result::Result<T, Box<Error>>;

fn setup() -> Result<()> {
    let mut a = try!(File::create("a"));
    try!(a.write_all(b"hello world.\n"));

    let mut b = try!(OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open("b"));
    b.write_all(b"rust programming language.\n").map_err(From::from)
}

fn get_data(path: &str) -> Result<String> {
    let mut file = try!( File::open(path) );

    let mut contents = String::new();
    try!( file.read_to_string(&mut contents) );

    Ok(contents)
}

fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (
        try!(get_data(a)),
        try!(get_data(b))
    );
    Ok(data_a + &data_b)
}

fn main() {
    match setup() {
        Err(ref e) => panic!("Error: {}", e.description()),
        _          => ()
    }

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}
