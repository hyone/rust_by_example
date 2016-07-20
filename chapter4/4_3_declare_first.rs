fn main() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding: u8;

    // // Error! use of uninitialized binding
    // println!("another_binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
