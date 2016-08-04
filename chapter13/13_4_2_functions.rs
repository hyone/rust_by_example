// One input reference with lifetime `'a`
// which must live at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Mutiple elements with different lifetimes.
// In this case, it would be fine for both to have the same lifetime `'a`,
// but in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }


// // Here, `&7` would create an `i32`, followed by a reference.
// // Then the data is dropped upon exiting the scope,
// // leaving a reference to invalid data to be returned
// fn invalid_output<'a>() -> &'a i32 { &7 }
// //=> error: borrowed value does not live long enough

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
