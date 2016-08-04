fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // *move* the ownership (and change mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // can modify the contents of the box
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    // println!("immutable_box contains {}", immutable_box);
    // //=> error: use of moved value: `immutable_box`
}
