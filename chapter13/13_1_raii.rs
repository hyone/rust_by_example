fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3132);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    let _box2 = Box::new(5132);

    {
        let _box3 = Box::new(4132);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun.
    // there's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
