#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// a tuple struct without attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // // `Seconds` can't be printed
    // println!("One second looks like: {:?}", _one_second);
    // //=> error: the trait bound `Seconds: std::fmt::Debug` is not satisfied [E0277]

    // // `Seconds` can't be compared
    // _one_second == _one_second;
    // //=> error: binary operation `==` cannot be applied to type `Seconds` [E0369]

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter", cmp);
}
