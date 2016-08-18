macro_rules! find_min {
    ($x: expr) => ($x);
    // `$x` is followed by at least one `$y,`
    ($x: expr, $($y: expr),+) => ({
        println!("{:?}", stringify!($($y),+));
        std::cmp::min($x, find_min!($($y),+))
    })
}

macro_rules! o_O {
    ( $( $x: expr; [$($y: expr),*] );* ) => {
        &[ $($($x + $y),*),* ]
    }
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32, 2u32));

    let a: &[i32] = o_O!(10; [1, 2, 3];
                         20; [4, 5, 6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}
