// A concrete type `A`
#[allow(dead_code)]
struct A { v: i32 }

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything,
// including the concrete type `A` defined at the top.
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A { v: 5 });

    // SingleGen has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // SingleGen can also have a type parameter implicitly specified.
    let _t    = SingleGen(A { v: 3 });
    let _i32  = SingleGen(6);
    let _char = SingleGen('a');
}
