// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing
    F: FnOnce() {
        f()
    }

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {
        f(3)
    }

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value;
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // mutation forces `farewell` to be captured by mutable reference.
        // So now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzzz");

        // manually calling `mem::drop` forces `farewell` to be captured by value.
        // So now requires `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubles: {}", apply_to_3(double));
}
