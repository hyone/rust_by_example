// suppress all warnings form casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // // Error! No Implicit Conversion
    // let integer: u8 = decimal;

    // Explicit Conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted
    // until the value fits the new type

    println!("1000 as a u16 is: {}", 1000 as u16);

    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("  -1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type.
    // If the most significant bit of that value is 1,
    // then the value is negative.

    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is: {}", 128 as i8);

    println!("1000 as a i8 is: {}", 1000 as i8);
    println!(" 232 as a i8 is: {}", 232 as i8);
}
