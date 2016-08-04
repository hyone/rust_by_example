// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn main() {
    // integer allocated on stack
    let x = 5u32;

    // *copy* `x` into `y`. no resources are moved
    let y = x;
    // both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a integer allocated on heap
    let a = Box::new(5132);
    println!("a contains: {}", a);

    // *move* `a` into `b`
    let b = a;
    // the pointer address of `a` is copied (not the data) into `b`.
    // both are now pointers to the same heap allocated data,
    // but `b` now owns it.

    // println!("a contains: {}", a);
    // //=> error: use of moved value: `a`

    println!("b contains: {}", b);

    // This fuction takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // // Since the heap allocated memory has been freed at this point,
    // // this action would result in dereferencing freed memory,
    // // but it's forbidden by the compiler
    // println!("b contains: {}", b);
    // //=> error: use of moved value: `b`
}
