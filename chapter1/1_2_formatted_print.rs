fn main() {
    // placeholder.
    // without a suffix, 31 becomes an i32.
    // you can change what type 31 is, with a suffix.
    println!("{} days", 31);

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // special formatting can be specified after ':'
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // right align number
    println!("{number:>width$}", number = 1, width = 6);

    // pad numbers with extra zeros
    println!("{number:>0width$}", number = 1, width = 6);
    //=> 000001

    // number of arguments is also checked at compile time
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure which contains an `i32`.
    // name it 'Structure'
    #[allow(dead_code)]
    struct Structure(i32);

    // // this will not work (don't satisfy traits required by println!)
    // println!("This struct `{}` won't print ...", Structure(3));

    // ------------------------------------------------------------------------
    // Activities
    // ------------------------------------------------------------------------

    let pi = 3.141592;
    println!("Pi is roughly #{pi:.3}", pi = pi);
}
