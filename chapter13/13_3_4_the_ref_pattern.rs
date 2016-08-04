#[derive(Clone, Copy, Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // "A `ref` borrow on the left side of an assignment"
    // is equivalent to
    // "an `&` borrow on the right side"
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` keyword can be used when destructing a struct
    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _ } = point;
        *ref_to_x;
    };

    // A mutable *copy* of `point`
    let mut mutable_point = point;

    mutable_point.x = 22;
    println!("mutable_point: {:?}, point: {:?}", mutable_point, point);

    {
        // take mutable reference via `ref mut`
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1;
    }
    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // let (mut first, ref mut last) = mutable_tuple;
        // //=> error: cannot bind by-move and by-ref in the same pattern
        let (ref mut first, ref mut last) = mutable_tuple;
        **first = 6u32;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
