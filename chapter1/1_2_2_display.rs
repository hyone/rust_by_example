use std::fmt;

struct Structure(i32);

// In order to use the `{}` marker,
// the trait `fmt::Display` must be implemented manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with thsi exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // - Write strictly the first element into the supplied output stream: `f`.
        //   Returns `fmt::Result` which indicates whether the operation succeeded or failed.
        // - `write!` uses syntax which is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("Structure(3): {}", Structure(3));
}
