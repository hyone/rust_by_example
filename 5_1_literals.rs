fn main() {
    use std::mem::{ size_of_val };

    // suffixed type annotation
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literal. their types depend on how they are used
    let i = 1;
    let f = 1.;

    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));
}
