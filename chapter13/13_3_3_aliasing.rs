struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the reference and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // let mutable_borrow = &mut point;
        // //=> error: cannot borrow `point` as mutable because it is also borrowed as immutable [E0502]

        // `borrowed_point` and `another_borrow` go out of the scope
    }

    {
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // let x = point.x;
        // //=> error: cannot use `point.x` because it was mutably borrowed

        // let y = &point.y;
        // //=> error: cannot borrow `point.y` as immutable because `point` is also borrowed as mutable [E0502]

        // `mutable_borrow` goes out of the scope
    }

    // immutable references to point are allowed again.
    println!("Point now has coordinates: ({}, {}, {})",
             point.x, point.y, point.z);
}
