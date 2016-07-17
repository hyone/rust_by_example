fn main() {
    // main function scope
    let long_lived_binding = 1;

    {
        // block scope
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // shodowing the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // // Error!
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // This binding also shadows the previous binding.
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
