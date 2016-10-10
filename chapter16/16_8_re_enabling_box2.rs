// the version that use `Error.description` in `Display.fmt`

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            DoubleError::EmptyVec =>
                "please use a vector with at least one element",
            DoubleError::Parse(ref e) =>
                e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DoubleError::EmptyVec     => None,
            DoubleError::Parse(ref e) => Some(e)
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first  = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
