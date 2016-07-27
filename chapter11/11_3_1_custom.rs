#[cfg(some_condition)]
fn conditional_function() {
    println!("some condition met!");
}
#[cfg(not(some_condition))]
fn conditional_function() {
    println!("none");
}

fn main() {
    conditional_function();
}
