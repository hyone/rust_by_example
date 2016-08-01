struct A;           // Concrete type `A`
struct S(A);        // Concrete type `S`
struct SGen<T>(T);  // Generic type `SGen`

// The following functions all take ownership of the variable passed into them
// and immediately go out of scope, freeing the variable.

// Define a function `reg_fn`
// that takes an argument `_s` of type `S`.
// it is not generic
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t`
// that takes an argument `_s` of type `SGen<T>`.
// it has been explicitly given the type parameter `A`,
// but because `A` has not been specified as a generic type parameter for `gen_spec_t`,
// it it not generic
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32`
// that takes an argument `_s` of type `SGen<i32>`.
// So, This is not also generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic`
// that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`,
// this function is genric over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter `char` to `generic()`
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SGen('c'));
}
