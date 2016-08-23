fn double_first(vec: Vec<&str>) -> i32 {
    // panic if the input vector is empty
    let first = vec.first().unwrap();

    // panic if the element doesn't parse to a number
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let strings = vec!["tofu", "cheese", "bell pepper"];
    let empty = vec![];

    println!("The first doubled is {}", double_first(numbers));

    // This line results in the first panic
    println!("The first doubled is {}", double_first(strings));
    //=> thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:788

    // This line results in the second panic
    println!("The first double is {}", double_first(empty));
    //=> thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:325
}
