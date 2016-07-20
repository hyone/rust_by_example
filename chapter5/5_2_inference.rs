fn main() {
    // because of the type annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    let mut vec = Vec::new();

    // At this point the compiler doesn't know the exact type of `vec`,
    // it just knows that it's a vector of something ( `Vec<_>` ) .

    // Insert `elem` in the vector.
    vec.push(elem);

    // Aha! Now that compiler knows that `vec` is a vector of `u8` ( `Vec<u8>` )

    println!("{:?}", vec);
}
