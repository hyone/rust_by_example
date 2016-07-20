#![allow(unused_variables)]

fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo { x: (1, 2), y: 3 };

    // destructing structs
    let Foo { x: (a, b), y } = foo;

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // ignore some variables
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // // this will give an error: pattern does not mention field `x`
    // let Foo { y } = foo;
}
