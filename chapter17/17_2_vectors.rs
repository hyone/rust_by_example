fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // // Immutable vector can't grow
    // collected_iterator.push(0);
    // //=> error: cannot borrow immutable local variable `collected_iterator` as mutable

    println!("Vector size: {}", xs.len());

    println!("Vector capacity: {}", xs.capacity());

    println!("Second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());

    // // Out of bounds indexing yields a panic at runtime
    // println!("Fourth element: {}", xs[3]);
    // //=> thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3', src/libcollections/vec.rs:1167

    // check out that the vector's capacity grows properly
    println!("Append 5 elements");
    xs.append(&mut vec![5, 6, 7, 8, 9]);
    println!("Vector: {:?}", xs);
    println!("Vector size: {}", xs.len());
    println!("Vector capacity: {}", xs.capacity());
}
