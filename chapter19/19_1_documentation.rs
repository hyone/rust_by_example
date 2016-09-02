// How to do doctest:
//
//   # NOTE: `19_1_documentation` is invalid module name,
//   #       so use symlink whose name is `documentation` instead.
//   $ rustc --crate-type=lib chapter19/documentation.rs
//   $ rustdoc --test --extern documentation=libdocumentation.rlib chapter19/documentation.rs
//
//   running 1 test
//   test Person::new_0 ... ok
//
//   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

/// A human being is represented here
#[derive(Debug)]
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// let person = documentation::Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

#[allow(dead_code)]
fn main() {
    let john = Person::new("John");
    john.hello();
}
