use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // Since `Length` implements `Copy`,
    // `add()` does not consume `one_foot` and `one_meter`
    // but copies from into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one_foot + one_foot = {:?}", two_feet);
    println!("one_meter + one_meter = {:?}", two_meters);

    // // one_foot and one_meter are actually difference types
    // let one_feter = one_foot + one_meter;
    // //=> error: mismatched types
}
