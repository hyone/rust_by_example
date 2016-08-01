use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]    // allow equality test for this type.
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: Storage is allocated for generic type `A`, but not for `B`.
//       Therefore, `B` cannnot be used in computations.

fn main() {
    // Here `f32` and `f64` are the hidden parameters.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // // tuple1 and tuple2 are actually different types
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // //=> error: mismatched types

    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
    // //=> error: mismatched types
}
