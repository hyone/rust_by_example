fn main() {
    // Assign a reference of type `i32`.
    // The `&` signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructing: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // In "let reference = &4",
    // `reference` was reference because rhs was already a reference.
    // This is not a reference because the right side is not one.
    let _not_a_reference = 3;
    // Rust provides `ref` for exactly this purpose.
    // `ref` modifies the assignment so that a reference is created for the element;
    // this reference is assigned.
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}
