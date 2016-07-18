// `NanoSecond` is a new name for `u64`
type NanoSecond = u64;
type Inch = u64;

// Non-CamelCase type names cause compiler warnings
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provie any extra type safety,
    // because aliases are *not* new types.
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
