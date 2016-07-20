// Derive the `fmt::Debug` implmentation
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
             "Slater",
             "Christian",
             actor = "actor's");

    println!("Now {:?} will print!", Structure(3));
    //=> Now Structure(3) will print!

    // The problem with `derive` is that
    // there is no control over how the results look.
    println!("Now {:?} will print!", Deep(Structure(7)));
    //=> Now Deep(Structure(7)) will print!
}
