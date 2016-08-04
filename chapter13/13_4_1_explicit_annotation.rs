// `print_refs` takes two references to `i32` which have different lifetimes `'a` and `'b`.
// These two lifetimes must both be at least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // `failed_borrow` contains no references to force `'a` to be longer than
    // the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static` .

    // // so the lifetime of `&_x` is shorter than that of `y`.
    // // A short lifetime cannot be coerced into a longer one.
    // let y: &'a i32 = &_x;
    // //=>  error: `_x` does not live long enough
}

fn main() {
    let (four, nine) = (4, 9);

    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be loger than that of `print_refs`.
    print_refs(&four, &nine);

    failed_borrow();
}
