fn double_number(number_str: &str) -> i32 {
    // It might not always be possible to parse a string into the other type,
    // so `parse()` return a `Result` indecating possible failure.
    // Let's try using `unwrap()` to get the number out.
    // Will it bite us?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    //=> thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:788
    println!("double is {}", tt);
}
