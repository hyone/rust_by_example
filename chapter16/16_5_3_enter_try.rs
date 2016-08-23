use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

type Result<T> = std::result::Result<T, String>;

fn setup() -> std::io::Result<()> {
    let mut a = try!(File::create("a"));
    try!(a.write_all(b"grape"));

    // open a file only when it has not existed yet, otherwise raise errors.
    let mut b = try!(OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open("b"));
    b.write_all(b"fruit");
}

fn get_data(path: &str) -> Result<String> {
    let mut file = try!(
        File::open(path).map_err(|e| e.to_string())
    );

    let mut contents = String::new();
    try!(
        file.read_to_string(&mut contents).map_err(|e| e.to_string())
    );

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
    // Ignore this result;
    let ret = setup();
    match ret {
        Err(ref e) => println!("Error: {}", e.to_string()),
        _          => (),
    }
    ret.unwrap();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e)
    }
}
