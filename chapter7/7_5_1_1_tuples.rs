#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    use Color::*;

    let color = CMYK(180, 30, 225, 40);
    // let color = HSV(55, 66, 77);
    // let color = RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Red          => println!("The color is Red!"),
        Blue         => println!("The color is Blue!"),
        Green        => println!("The color is Green!"),
        RGB(r, g, b) =>
            println!("Red: {:02x}, green: {:02x}, blue: {:02x}", r, g, b),
        HSV(h, s, v) =>
            println!("Hue: {:02x}, saturation: {:02x}, value: {:02x}", h, s, v),
        CMYK(c, m, y, k) =>
            println!("Cyan: {:02x}, magenta: {:02x}, yellow: {:02x}, key (black): {:02x}",
                     c, m, y, k),
    }
}
