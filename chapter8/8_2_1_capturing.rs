fn main() {
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count` or `count`.
    // but `&mut count` is less restrictive so it takes that.
    // Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // // Error: cannot borrow `count` as mutable more than once at a time
    // let reborrow = &mut count;

    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value.
    // A copy type would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` Immediately moves into the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // // Error: use of moved value: `consume`
    // consume();
}
