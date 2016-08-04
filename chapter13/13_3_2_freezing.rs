fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Borrow `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // _mutable_integer = 50;
        // //=> error: cannot assign to `_mutable_integer` because it is borrowed

        // `_large_integer` goes out of the scope
    }

    // OK! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
