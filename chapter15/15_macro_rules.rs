macro_rules! say_hello {
    () => (
        println!("Hello!");
    )
}

fn main() {
    // This call will expand into `println("Hello");`
    say_hello!();
}
