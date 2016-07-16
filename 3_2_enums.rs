// hide warnings for unused code.
// #![allow(dead_code)]

enum Person {
    // An `enum` may either be
    // like unit,
    Engineer,
    Scientist,
    // like tuple structs,
    Height(i32),
    Weight(i32),
    // like structs
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    match p {
        Person::Engineer  => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person  = Person::Height(18);
    let amira   = Person::Weight(10);
    // to_owned(): creates an owned `String` from a string slice
    let dave    = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan   = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
