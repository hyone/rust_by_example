use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // // `HashSet::insert` returns false if there was a value already present.
    // assert!(b.insert(4), "Value 4 is already in set B!");
    // //=> thread 'main' panicked at 'Value 4 is already in set B!'

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    //=> Union: [3, 4, 1, 2, 5]

    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    //=> Difference: [1]

    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    //=> Intersection: [3, 4, 2]

    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
    //=> Symmetric Difference: [1, 5]
}
