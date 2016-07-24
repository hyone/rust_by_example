fn main() {
    fn function(i: i32) -> i32 { i + 1 }

    // Annotation is identical to function annotation
    // but is optional as are the `{}` wrapping the body.
    // These nameless functions are assigned to appropriately name variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // A closure taking no arguments, which returns an `i32`.
    let one = || 1;
    println!("closure returning one: {}", one());

    // Closures capture variables from the enclosing scope,
    // which is impossible with functions.
    let professor_x = "Charles Xavier";
    let print = || println!("Professor X's name is: {}", professor_x);
    print();

    // Functions can't capture variables from the enclosing scope.
    // // Error!
    // // =>  error: can't capture dynamic environment in a fn item; use the || { ... } closure form instead [E0434]
    // fn print2() {
    //     println!("Professor X's name is: {}", professor_x);
    // }
}
