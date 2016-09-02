fn main() {
    let reference: &u32 = &10;
    let raw_pointer: *const u32 = &10;

    println!("{}", *reference);

    // println!("{}", *raw_pointer);
    // //=> error: dereference of raw pointer requires unsafe function or block [E0133]

    unsafe {
        println!("{}", *raw_pointer);
    }
}
